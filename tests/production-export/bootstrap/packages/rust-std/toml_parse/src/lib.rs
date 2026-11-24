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
    C0(List<Bytes>, bool, List<List<(Bytes, Adt1)>>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes),
    C1(bool),
    C2(u64),
    C3(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt3>, List<Bytes>, bool, List<(Bytes, Adt1)>, Option<Bytes>, List<Bytes>, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(List<Bytes>, bool, List<(Bytes, Adt1)>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, Bytes, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F2,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2, p1: Bytes) -> R<Adt2> {
        match self {
            Fn0::F2 => f2(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, List<Adt0>), p1: Adt3) -> (bool, List<Adt0>) {
        match self {
            Fn1::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F10(Bytes),
    F60(u8, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn2::F10(k0) => f10(k0.clone(), p0),
            Fn2::F60(k0, k1) => f60(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F14,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn3::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F23,
    F31,
    F39,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn4::F23 => f23(p0, p1),
            Fn4::F31 => f31(p0, p1),
            Fn4::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F33,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn5::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F41(Bytes, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn6::F41(k0, k1) => f41(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F44(Bytes, u64, Bytes, u64),
    F57(Bytes),
    F58(u64, Bytes),
    F79(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F44(k0, k1, k2, k3) => f44(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn7::F57(k0) => Ok(f57(k0.clone(), p0)),
            Fn7::F58(k0, k1) => f58(*k0, k1.clone(), p0),
            Fn7::F79(k0) => f79(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F47(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt1)) -> bool {
        match self {
            Fn8::F47(k0) => f47(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F53(bool, List<Bytes>),
}

impl Fn9 {
    pub fn apply(&self, p0: bool, p1: Adt0) -> bool {
        match self {
            Fn9::F53(k0, k1) => f53(*k0, k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F56(bool, List<Bytes>, List<(Bytes, Adt1)>),
    F67,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn10::F56(k0, k1, k2) => f56(*k0, k1.clone(), k2.clone(), p0, p1),
            Fn10::F67 => f67(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F62(Bytes, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn11::F62(k0, k1) => f62(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F65(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn12::F65(k0) => f65(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F83,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn13::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F86(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn14::F86(k0) => f86(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<List<Adt0>>> {
    let v1: Adt2 = f1(Fn0::F2, Adt2::C0(List::<Adt3>::nil(), List::<Bytes>::nil(), false, List::<(Bytes, Adt1)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false), f3(v0)?)?;
    if match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h1) => {
            h1
        }
    } {
        Ok(None::<List<Adt0>>)
    } else {
        let m3 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2, _, _) => {
                h2
            }
        };
        match m3 {
            None => {
                let v3: List<Adt3> = f4(f5(match v1.clone() {
                    Adt2::C0(h4, _, _, _, _, _, _) => {
                        h4
                    }
                }, match v1.clone() {
                    Adt2::C0(_, h5, _, _, _, _, _) => {
                        h5
                    }
                }, match v1.clone() {
                    Adt2::C0(_, _, h6, _, _, _, _) => {
                        h6
                    }
                }, match v1 {
                    Adt2::C0(_, _, _, h7, _, _, _) => {
                        h7
                    }
                }));
                let v4: (bool, List<Adt0>) = f6(Fn1::F7, (false, List::<Adt0>::nil()), v3);
                let (h8, _) = v4;
                if h8 {
                    Ok(None::<List<Adt0>>)
                } else {
                    Ok(Some({
                        let (_, h9) = v4;
                        h9
                    }))
                }
            }
            Some(_) => {
                Ok(None::<List<Adt0>>)
            }
        }
    }
}

pub fn f1(v0: Fn0, p13: Adt2, p14: List<Bytes>) -> R<Adt2> {
    let m12 = (p13, p14);
    {
        let mut m12 = m12;
        loop {
            let step = {
                let (v1, v2) = m12;
                let m10 = v2;
                match m10.uncons() {
                    None => {
                        Err::<(Adt2, List<Bytes>), R<Adt2>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt2, List<Bytes>), R<Adt2>>(({
                            let c11: Fn0 = v0.clone();
                            c11.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m12 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: Adt2, v1: Bytes) -> R<Adt2> {
    f8(v0, v1)
}

pub fn f3(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a15 = v0.clone();
        length_bytes(a15)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h16, _) = f9(Fn2::F10(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h16
    };
    let v3: u64 = {
        let (h17, _) = {
            let (_, h18) = v2.clone();
            h18
        };
        h17
    };
    let v4: List<Bytes> = {
        let (_, h19) = {
            let (_, h20) = v2;
            h20
        };
        h19
    };
    let a21 = v3;
    let a22 = v1;
    if nat_lt(a21, a22) {
        Ok(f11(List::cons(f12(v0, v3, {
            let a23 = v1;
            let a24 = v3;
            nat_sub(a23, a24)
        }), v4)))
    } else {
        Ok(f11(v4))
    }
}

pub fn f4(v0: List<Adt3>) -> List<Adt3> {
    f13(Fn3::F14, List::<Adt3>::nil(), v0)
}

pub fn f5(v0: List<Adt3>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt1)>) -> List<Adt3> {
    List::cons(Adt3::C0(v1, v2, f15(v3)), v0)
}

pub fn f6(v0: Fn1, p28: (bool, List<Adt0>), p29: List<Adt3>) -> (bool, List<Adt0>) {
    let m27 = (p28, p29);
    {
        let mut m27 = m27;
        loop {
            let step = {
                let (v1, v2) = m27;
                let m25 = v2;
                match m25.uncons() {
                    None => {
                        Err::<((bool, List<Adt0>), List<Adt3>), (bool, List<Adt0>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Adt0>), List<Adt3>), (bool, List<Adt0>)>(({
                            let c26: Fn1 = v0.clone();
                            c26.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m27 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: (bool, List<Adt0>), v1: Adt3) -> (bool, List<Adt0>) {
    f16(v0, v1)
}

pub fn f8(v0: Adt2, v1: Bytes) -> R<Adt2> {
    if match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h30) => {
            h30
        }
    } {
        Ok(v0)
    } else {
        let m32 = match v0.clone() {
            Adt2::C0(_, _, _, _, h31, _, _) => {
                h31
            }
        };
        match m32 {
            None => {
                if f24(v1.clone())? {
                    Ok(v0)
                } else {
                    let v8: Bytes = f17(v1)?;
                    if f25(v8.clone(), f18(Str::lit("["))) {
                        let v9: List<Adt3> = f5(match v0.clone() {
                            Adt2::C0(h59, _, _, _, _, _, _) => {
                                h59
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, h60, _, _, _, _, _) => {
                                h60
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h61, _, _, _, _) => {
                                h61
                            }
                        }, match v0 {
                            Adt2::C0(_, _, _, h62, _, _, _) => {
                                h62
                            }
                        });
                        let m63 = f26(v8)?;
                        match m63 {
                            None => {
                                Ok(Adt2::C0(v9, List::<Bytes>::nil(), false, List::<(Bytes, Adt1)>::nil(), None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v10) => {
                                Ok(Adt2::C0(v9, {
                                    let (h64, _) = v10;
                                    h64
                                }, {
                                    let (_, h65) = v10;
                                    h65
                                }, List::<(Bytes, Adt1)>::nil(), None::<Bytes>, List::<Bytes>::nil(), false))
                            }
                        }
                    } else {
                        let m66 = f27(v8.clone(), f18(Str::lit("=")))?;
                        match m66 {
                            None => {
                                Ok(Adt2::C0(match v0.clone() {
                                    Adt2::C0(h67, _, _, _, _, _, _) => {
                                        h67
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h68, _, _, _, _, _) => {
                                        h68
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, _, h69, _, _, _, _) => {
                                        h69
                                    }
                                }, match v0 {
                                    Adt2::C0(_, _, _, h70, _, _, _) => {
                                        h70
                                    }
                                }, None::<Bytes>, List::<Bytes>::nil(), true))
                            }
                            Some(v11) => {
                                let v12: Bytes = f17(f12(v8.clone(), 0u64, v11))?;
                                let v13: Bytes = f17(f12(v8.clone(), {
                                    let a71 = v11;
                                    let a72 = 1u64;
                                    nat_add(a71, a72)?
                                }, {
                                    let a76 = {
                                        let a73 = v8;
                                        length_bytes(a73)
                                    };
                                    let a77 = {
                                        let a74 = v11;
                                        let a75 = 1u64;
                                        nat_add(a74, a75)?
                                    };
                                    nat_sub(a76, a77)
                                }))?;
                                let a79 = {
                                    let a78 = v12.clone();
                                    length_bytes(a78)
                                };
                                let a80 = 0u64;
                                if if nat_eq(a79, a80) {
                                    true
                                } else {
                                    let m81 = f27(v12.clone(), f18(Str::lit(".")))?;
                                    match m81 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                } {
                                    Ok(Adt2::C0(match v0.clone() {
                                        Adt2::C0(h82, _, _, _, _, _, _) => {
                                            h82
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, h83, _, _, _, _, _) => {
                                            h83
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, _, h84, _, _, _, _) => {
                                            h84
                                        }
                                    }, match v0 {
                                        Adt2::C0(_, _, _, h85, _, _, _) => {
                                            h85
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                } else {
                                    if f28(match v0.clone() {
                                        Adt2::C0(_, _, _, h86, _, _, _) => {
                                            h86
                                        }
                                    }, v12.clone()) {
                                        Ok(Adt2::C0(match v0.clone() {
                                            Adt2::C0(h87, _, _, _, _, _, _) => {
                                                h87
                                            }
                                        }, match v0.clone() {
                                            Adt2::C0(_, h88, _, _, _, _, _) => {
                                                h88
                                            }
                                        }, match v0.clone() {
                                            Adt2::C0(_, _, h89, _, _, _, _) => {
                                                h89
                                            }
                                        }, match v0 {
                                            Adt2::C0(_, _, _, h90, _, _, _) => {
                                                h90
                                            }
                                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                                    } else {
                                        f29(v0, v12, v13)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(v2) => {
                let v3: Bytes = f17(v1)?;
                let a33 = v3.clone();
                let a34 = f18(Str::lit("]"));
                if equal(a33, a34) {
                    Ok(Adt2::C0(match v0.clone() {
                        Adt2::C0(h35, _, _, _, _, _, _) => {
                            h35
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, h36, _, _, _, _, _) => {
                            h36
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, h37, _, _, _, _) => {
                            h37
                        }
                    }, List::cons((v2, Adt1::C3(f19(match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h38, _) => {
                            h38
                        }
                    }))), match v0 {
                        Adt2::C0(_, _, _, h39, _, _, _) => {
                            h39
                        }
                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                } else {
                    let v4: bool = f20(v3.clone(), f18(Str::lit("]")));
                    let v5: Bytes = if v4 {
                        f12(v3.clone(), 0u64, {
                            let a41 = {
                                let a40 = v3;
                                length_bytes(a40)
                            };
                            let a42 = 1u64;
                            nat_sub(a41, a42)
                        })
                    } else {
                        v3
                    };
                    let m43 = f21(v5)?;
                    match m43 {
                        None => {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h53, _, _, _, _, _, _) => {
                                    h53
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h54, _, _, _, _, _) => {
                                    h54
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h55, _, _, _, _) => {
                                    h55
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, h56, _, _, _) => {
                                    h56
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, h57, _, _) => {
                                    h57
                                }
                            }, match v0 {
                                Adt2::C0(_, _, _, _, _, h58, _) => {
                                    h58
                                }
                            }, true))
                        }
                        Some(v6) => {
                            let v7: List<Bytes> = f22(Fn4::F23, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h44, _) => {
                                    h44
                                }
                            }, v6);
                            if v4 {
                                Ok(Adt2::C0(match v0.clone() {
                                    Adt2::C0(h45, _, _, _, _, _, _) => {
                                        h45
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h46, _, _, _, _, _) => {
                                        h46
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, _, h47, _, _, _, _) => {
                                        h47
                                    }
                                }, List::cons((v2, Adt1::C3(f19(v7))), match v0 {
                                    Adt2::C0(_, _, _, h48, _, _, _) => {
                                        h48
                                    }
                                }), None::<Bytes>, List::<Bytes>::nil(), false))
                            } else {
                                Ok(Adt2::C0(match v0.clone() {
                                    Adt2::C0(h49, _, _, _, _, _, _) => {
                                        h49
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h50, _, _, _, _, _) => {
                                        h50
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, _, h51, _, _, _, _) => {
                                        h51
                                    }
                                }, match v0 {
                                    Adt2::C0(_, _, _, h52, _, _, _) => {
                                        h52
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

pub fn f9(v0: Fn2, p95: u64, p96: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m91 = v1;
                if m91 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m91 - 1;
                    let m93 = {
                        let c92: Fn2 = v0.clone();
                        c92.apply(v2.clone())?
                    };
                    match m93 {
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
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f30(v0, v1)
}

pub fn f11(v0: List<Bytes>) -> List<Bytes> {
    f22(Fn4::F31, List::<Bytes>::nil(), v0)
}

pub fn f12(v0: Bytes, v1: u64, v2: u64) -> Bytes {
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

pub fn f13(v0: Fn3, p104: List<Adt3>, p105: List<Adt3>) -> List<Adt3> {
    let m103 = (p104, p105);
    {
        let mut m103 = m103;
        loop {
            let step = {
                let (v1, v2) = m103;
                let m101 = v2;
                match m101.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c102: Fn3 = v0.clone();
                            c102.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f15(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f32(Fn5::F33, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f16(v0: (bool, List<Adt0>), v1: Adt3) -> (bool, List<Adt0>) {
    let (h106, _) = v0.clone();
    if h106 {
        v0
    } else {
        let v2: List<Bytes> = match v1.clone() {
            Adt3::C0(h107, _, _) => {
                h107
            }
        };
        let v3: bool = match v1.clone() {
            Adt3::C0(_, h108, _) => {
                h108
            }
        };
        let v4: List<(Bytes, Adt1)> = match v1 {
            Adt3::C0(_, _, h109) => {
                h109
            }
        };
        let v5: bool = f34({
            let (_, h110) = v0.clone();
            h110
        }, v2.clone(), v3);
        if v5 {
            if v3 {
                (false, f35({
                    let (_, h111) = v0;
                    h111
                }, v2, v3, v4))
            } else {
                (true, {
                    let (_, h112) = v0;
                    h112
                })
            }
        } else {
            (false, f36({
                let (_, h113) = v0;
                h113
            }, v2, v3, v4))
        }
    }
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f37(v0.clone())?;
    let v2: u64 = f38(v0.clone(), v1)?;
    Ok(f12(v0, v2, {
        let a114 = v1;
        let a115 = v2;
        nat_sub(a114, a115)
    }))
}

pub fn f18(v0: Str) -> Bytes {
    let a116 = v0;
    utf8_encode(a116)
}

pub fn f19(v0: List<Bytes>) -> List<Bytes> {
    f22(Fn4::F39, List::<Bytes>::nil(), v0)
}

pub fn f20(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a117 = v0.clone();
        length_bytes(a117)
    };
    let v3: u64 = {
        let a118 = v1.clone();
        length_bytes(a118)
    };
    let a119 = v3;
    let a120 = v2;
    if nat_le(a119, a120) {
        let m126 = {
            let a123 = v0;
            let a124 = {
                let a121 = v2;
                let a122 = v3;
                nat_sub(a121, a122)
            };
            let a125 = v3;
            slice_bytes(a123, a124, a125)
        };
        match m126 {
            None => {
                false
            }
            Some(v4) => {
                let a127 = v4;
                let a128 = v1;
                equal(a127, a128)
            }
        }
    } else {
        false
    }
}

pub fn f21(v0: Bytes) -> R<Option<List<Bytes>>> {
    let v1: u64 = {
        let a129 = v0.clone();
        length_bytes(a129)
    };
    let v2: Adt4 = {
        let (h130, _) = f40(Fn6::F41(v0, v1), {
            let a131 = v1;
            let a132 = 2u64;
            nat_add(a131, a132)?
        }, Adt4::C0(0u64, List::<Bytes>::nil(), 0u64))?;
        h130
    };
    let a134 = match v2.clone() {
        Adt4::C0(_, _, h133) => {
            h133
        }
    };
    let a135 = 1u64;
    if equal(a134, a135) {
        Ok(Some(f19(match v2 {
            Adt4::C0(_, h136, _) => {
                h136
            }
        })))
    } else {
        Ok(None::<List<Bytes>>)
    }
}

pub fn f22(v0: Fn4, p140: List<Bytes>, p141: List<Bytes>) -> List<Bytes> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c138: Fn4 = v0.clone();
                            c138.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f24(v0: Bytes) -> R<bool> {
    let v1: Bytes = f17(v0)?;
    let a143 = {
        let a142 = v1.clone();
        length_bytes(a142)
    };
    let a144 = 0u64;
    if nat_eq(a143, a144) {
        Ok(true)
    } else {
        Ok(f25(v1, f18(Str::lit("#"))))
    }
}

pub fn f25(v0: Bytes, v1: Bytes) -> bool {
    let m149 = {
        let a146 = v0;
        let a147 = 0u64;
        let a148 = {
            let a145 = v1.clone();
            length_bytes(a145)
        };
        slice_bytes(a146, a147, a148)
    };
    match m149 {
        None => {
            false
        }
        Some(v2) => {
            let a150 = v2;
            let a151 = v1;
            equal(a150, a151)
        }
    }
}

pub fn f26(v0: Bytes) -> R<Option<(List<Bytes>, bool)>> {
    let v1: bool = f25(v0.clone(), f18(Str::lit("[[")));
    let v2: u64 = {
        let a152 = v0.clone();
        length_bytes(a152)
    };
    let v3: Bytes = if v1 {
        f12(v0, 2u64, {
            let a153 = v2;
            let a154 = 2u64;
            nat_sub(a153, a154)
        })
    } else {
        f12(v0, 1u64, {
            let a155 = v2;
            let a156 = 1u64;
            nat_sub(a155, a156)
        })
    };
    let v4: bool = if v1 {
        f20(v3.clone(), f18(Str::lit("]]")))
    } else {
        f20(v3.clone(), f18(Str::lit("]")))
    };
    if v4 {
        let v5: u64 = {
            let a157 = v3.clone();
            length_bytes(a157)
        };
        let v6: u64 = if v1 {
            2u64
        } else {
            1u64
        };
        let v7: Bytes = f12(v3, 0u64, {
            let a158 = v5;
            let a159 = v6;
            nat_sub(a158, a159)
        });
        let a161 = {
            let a160 = v7.clone();
            length_bytes(a160)
        };
        let a162 = 0u64;
        if nat_eq(a161, a162) {
            Ok(None::<(List<Bytes>, bool)>)
        } else {
            Ok(Some((f42(v7, 46u8)?, v1)))
        }
    } else {
        Ok(None::<(List<Bytes>, bool)>)
    }
}

pub fn f27(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a163 = v0.clone();
        length_bytes(a163)
    };
    let v3: u64 = {
        let a164 = v1.clone();
        length_bytes(a164)
    };
    let v4: u64 = {
        let (h165, _) = f43(Fn7::F44(v1.clone(), v3, v0.clone(), v2), {
            let a166 = v2;
            let a167 = 1u64;
            nat_add(a166, a167)?
        }, 0u64)?;
        h165
    };
    if f45(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f28(v0: List<(Bytes, Adt1)>, v1: Bytes) -> bool {
    f46(Fn8::F47(v1), false, v0)
}

pub fn f29(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Adt2> {
    let a168 = v2.clone();
    let a169 = f18(Str::lit("true"));
    if equal(a168, a169) {
        Ok(Adt2::C0(match v0.clone() {
            Adt2::C0(h170, _, _, _, _, _, _) => {
                h170
            }
        }, match v0.clone() {
            Adt2::C0(_, h171, _, _, _, _, _) => {
                h171
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h172, _, _, _, _) => {
                h172
            }
        }, List::cons((v1, Adt1::C1(true)), match v0 {
            Adt2::C0(_, _, _, h173, _, _, _) => {
                h173
            }
        }), None::<Bytes>, List::<Bytes>::nil(), false))
    } else {
        let a174 = v2.clone();
        let a175 = f18(Str::lit("false"));
        if equal(a174, a175) {
            Ok(Adt2::C0(match v0.clone() {
                Adt2::C0(h176, _, _, _, _, _, _) => {
                    h176
                }
            }, match v0.clone() {
                Adt2::C0(_, h177, _, _, _, _, _) => {
                    h177
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h178, _, _, _, _) => {
                    h178
                }
            }, List::cons((v1, Adt1::C1(false)), match v0 {
                Adt2::C0(_, _, _, h179, _, _, _) => {
                    h179
                }
            }), None::<Bytes>, List::<Bytes>::nil(), false))
        } else {
            if f25(v2.clone(), f18(Str::lit("\""))) {
                let m180 = f48(v2.clone(), 0u64)?;
                match m180 {
                    None => {
                        Ok(Adt2::C0(match v0.clone() {
                            Adt2::C0(h194, _, _, _, _, _, _) => {
                                h194
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, h195, _, _, _, _, _) => {
                                h195
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h196, _, _, _, _) => {
                                h196
                            }
                        }, match v0 {
                            Adt2::C0(_, _, _, h197, _, _, _) => {
                                h197
                            }
                        }, None::<Bytes>, List::<Bytes>::nil(), true))
                    }
                    Some(v3) => {
                        let a183 = {
                            let (_, h181) = v3;
                            h181
                        };
                        let a184 = {
                            let a182 = v2;
                            length_bytes(a182)
                        };
                        if nat_eq(a183, a184) {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h185, _, _, _, _, _, _) => {
                                    h185
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h186, _, _, _, _, _) => {
                                    h186
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h187, _, _, _, _) => {
                                    h187
                                }
                            }, List::cons((v1, Adt1::C0({
                                let (h188, _) = v3;
                                h188
                            })), match v0 {
                                Adt2::C0(_, _, _, h189, _, _, _) => {
                                    h189
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        } else {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h190, _, _, _, _, _, _) => {
                                    h190
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h191, _, _, _, _, _) => {
                                    h191
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h192, _, _, _, _) => {
                                    h192
                                }
                            }, match v0 {
                                Adt2::C0(_, _, _, h193, _, _, _) => {
                                    h193
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                }
            } else {
                if f25(v2.clone(), f18(Str::lit("["))) {
                    let a198 = v2.clone();
                    let a199 = f18(Str::lit("["));
                    if equal(a198, a199) {
                        Ok(Adt2::C0(match v0.clone() {
                            Adt2::C0(h200, _, _, _, _, _, _) => {
                                h200
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, h201, _, _, _, _, _) => {
                                h201
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h202, _, _, _, _) => {
                                h202
                            }
                        }, match v0 {
                            Adt2::C0(_, _, _, h203, _, _, _) => {
                                h203
                            }
                        }, Some(v1), List::<Bytes>::nil(), false))
                    } else {
                        if f20(v2.clone(), f18(Str::lit("]"))) {
                            let m207 = f21(f12(v2.clone(), 1u64, {
                                let a205 = {
                                    let a204 = v2;
                                    length_bytes(a204)
                                };
                                let a206 = 2u64;
                                nat_sub(a205, a206)
                            }))?;
                            match m207 {
                                None => {
                                    Ok(Adt2::C0(match v0.clone() {
                                        Adt2::C0(h212, _, _, _, _, _, _) => {
                                            h212
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, h213, _, _, _, _, _) => {
                                            h213
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, _, h214, _, _, _, _) => {
                                            h214
                                        }
                                    }, match v0 {
                                        Adt2::C0(_, _, _, h215, _, _, _) => {
                                            h215
                                        }
                                    }, None::<Bytes>, List::<Bytes>::nil(), true))
                                }
                                Some(v4) => {
                                    Ok(Adt2::C0(match v0.clone() {
                                        Adt2::C0(h208, _, _, _, _, _, _) => {
                                            h208
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, h209, _, _, _, _, _) => {
                                            h209
                                        }
                                    }, match v0.clone() {
                                        Adt2::C0(_, _, h210, _, _, _, _) => {
                                            h210
                                        }
                                    }, List::cons((v1, Adt1::C3(v4)), match v0 {
                                        Adt2::C0(_, _, _, h211, _, _, _) => {
                                            h211
                                        }
                                    }), None::<Bytes>, List::<Bytes>::nil(), false))
                                }
                            }
                        } else {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h216, _, _, _, _, _, _) => {
                                    h216
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h217, _, _, _, _, _) => {
                                    h217
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h218, _, _, _, _) => {
                                    h218
                                }
                            }, match v0 {
                                Adt2::C0(_, _, _, h219, _, _, _) => {
                                    h219
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                    }
                } else {
                    let m220 = f49(v2)?;
                    match m220 {
                        None => {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h225, _, _, _, _, _, _) => {
                                    h225
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h226, _, _, _, _, _) => {
                                    h226
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h227, _, _, _, _) => {
                                    h227
                                }
                            }, match v0 {
                                Adt2::C0(_, _, _, h228, _, _, _) => {
                                    h228
                                }
                            }, None::<Bytes>, List::<Bytes>::nil(), true))
                        }
                        Some(v5) => {
                            Ok(Adt2::C0(match v0.clone() {
                                Adt2::C0(h221, _, _, _, _, _, _) => {
                                    h221
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, h222, _, _, _, _, _) => {
                                    h222
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, h223, _, _, _, _) => {
                                    h223
                                }
                            }, List::cons((v1, Adt1::C2(v5)), match v0 {
                                Adt2::C0(_, _, _, h224, _, _, _) => {
                                    h224
                                }
                            }), None::<Bytes>, List::<Bytes>::nil(), false))
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h229, _) = v1.clone();
        h229
    };
    let v3: u64 = {
        let (h230, _) = {
            let (_, h231) = v1.clone();
            h231
        };
        h230
    };
    let v4: List<Bytes> = {
        let (_, h232) = {
            let (_, h233) = v1;
            h233
        };
        h232
    };
    let m234 = f50(v0.clone(), v2);
    match m234 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a235 = v5;
            let a236 = 10u8;
            if equal(a235, a236) {
                Ok(Some(({
                    let a237 = v2;
                    let a238 = 1u64;
                    nat_add(a237, a238)?
                }, ({
                    let a239 = v2;
                    let a240 = 1u64;
                    nat_add(a239, a240)?
                }, List::cons(f51(f12(v0, v3, {
                    let a241 = v2;
                    let a242 = v3;
                    nat_sub(a241, a242)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a243 = v2;
                    let a244 = 1u64;
                    nat_add(a243, a244)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f31(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f32(v0: Fn5, p248: List<(Bytes, Adt1)>, p249: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m245 = v2;
                match m245.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c246: Fn5 = v0.clone();
                            c246.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f34(v0: List<Adt0>, v1: List<Bytes>, v2: bool) -> bool {
    f52(Fn9::F53(v2, v1), false, v0)
}

pub fn f35(v0: List<Adt0>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt1)>) -> List<Adt0> {
    f54(f55(Fn10::F56(v2, v1, v3), List::<Adt0>::nil(), v0))
}

pub fn f36(v0: List<Adt0>, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt1)>) -> List<Adt0> {
    f54(List::cons(Adt0::C0(v1, v2, List::cons(v3, List::<List<(Bytes, Adt1)>>::nil())), f54(v0)))
}

pub fn f37(v0: Bytes) -> R<u64> {
    let (h250, _) = f43(Fn7::F57(v0.clone()), {
        let a251 = v0.clone();
        length_bytes(a251)
    }, {
        let a252 = v0;
        length_bytes(a252)
    })?;
    Ok(h250)
}

pub fn f38(v0: Bytes, v1: u64) -> R<u64> {
    let (h253, _) = f43(Fn7::F58(v1, v0), v1, 0u64)?;
    Ok(h253)
}

pub fn f39(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f40(v0: Fn6, p258: u64, p259: Adt4) -> R<(Adt4, bool)> {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m254 = v1;
                if m254 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m254 - 1;
                    let m256 = {
                        let c255: Fn6 = v0.clone();
                        c255.apply(v2.clone())?
                    };
                    match m256 {
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
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    f59(v0, v1, v2)
}

pub fn f42(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a260 = v0.clone();
        length_bytes(a260)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h261, _) = f9(Fn2::F60(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h261
    };
    let v4: u64 = {
        let (h262, _) = {
            let (_, h263) = v3.clone();
            h263
        };
        h262
    };
    Ok(f11(List::cons(f12(v0, v4, {
        let a264 = v2;
        let a265 = v4;
        nat_sub(a264, a265)
    }), {
        let (_, h266) = {
            let (_, h267) = v3;
            h267
        };
        h266
    })))
}

pub fn f43(v0: Fn7, p272: u64, p273: u64) -> R<(u64, bool)> {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v1, v2) = m271;
                let m268 = v1;
                if m268 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m268 - 1;
                    let m270 = {
                        let c269: Fn7 = v0.clone();
                        c269.apply(v2)?
                    };
                    match m270 {
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
                Ok(next) => { m271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f45(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a276 = {
            let a274 = v4;
            let a275 = v1;
            nat_add(a274, a275)?
        };
        let a277 = v3;
        if nat_lt(a276, a277) {
            Ok(Some({
                let a278 = v4;
                let a279 = 1u64;
                nat_add(a278, a279)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f45(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m284 = {
        let a281 = v0;
        let a282 = v2;
        let a283 = {
            let a280 = v1.clone();
            length_bytes(a280)
        };
        slice_bytes(a281, a282, a283)
    };
    match m284 {
        None => {
            false
        }
        Some(v3) => {
            let a285 = v3;
            let a286 = v1;
            equal(a285, a286)
        }
    }
}

pub fn f46(v0: Fn8, p290: bool, p291: List<(Bytes, Adt1)>) -> bool {
    let m289 = (p290, p291);
    {
        let mut m289 = m289;
        loop {
            let step = {
                let (v1, v2) = m289;
                let m287 = v2;
                match m287.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt1)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt1)>), bool>(({
                            let c288: Fn8 = v0.clone();
                            c288.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: bool, v2: (Bytes, Adt1)) -> bool {
    if v1 {
        true
    } else {
        let a293 = {
            let (h292, _) = v2;
            h292
        };
        let a294 = v0;
        equal(a293, a294)
    }
}

pub fn f48(v0: Bytes, v1: u64) -> R<Option<(Bytes, u64)>> {
    let m295 = f50(v0.clone(), v1);
    match m295 {
        None => {
            Ok(None::<(Bytes, u64)>)
        }
        Some(v2) => {
            let a296 = v2;
            let a297 = 34u8;
            if equal(a296, a297) {
                let v3: u64 = {
                    let a298 = v0.clone();
                    length_bytes(a298)
                };
                let v4: Adt5 = {
                    let (h299, _) = f61(Fn11::F62(v0, v3), {
                        let a300 = v3;
                        let a301 = 2u64;
                        nat_add(a300, a301)?
                    }, Adt5::C0({
                        let a302 = v1;
                        let a303 = 1u64;
                        nat_add(a302, a303)?
                    }, Bytes::lit(&[]), 0u64, false))?;
                    h299
                };
                if match v4.clone() {
                    Adt5::C0(_, _, _, h304) => {
                        h304
                    }
                } {
                    Ok(Some((match v4.clone() {
                        Adt5::C0(_, h305, _, _) => {
                            h305
                        }
                    }, match v4 {
                        Adt5::C0(h306, _, _, _) => {
                            h306
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

pub fn f49(v0: Bytes) -> R<Option<u64>> {
    if f63(v0.clone())? {
        Ok(Some({
            let (_, h307) = {
                let (h308, _) = f64(Fn12::F65(v0.clone()), {
                    let a309 = v0;
                    length_bytes(a309)
                }, (0u64, 0u64))?;
                h308
            };
            h307
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f50(v0: Bytes, v1: u64) -> Option<u8> {
    let a310 = v0;
    let a311 = v1;
    index_bytes(a310, a311)
}

pub fn f51(v0: Bytes) -> Bytes {
    if f20(v0.clone(), Bytes::lit(&[0x0d])) {
        f12(v0.clone(), 0u64, {
            let a313 = {
                let a312 = v0;
                length_bytes(a312)
            };
            let a314 = 1u64;
            nat_sub(a313, a314)
        })
    } else {
        v0
    }
}

pub fn f52(v0: Fn9, p318: bool, p319: List<Adt0>) -> bool {
    let m317 = (p318, p319);
    {
        let mut m317 = m317;
        loop {
            let step = {
                let (v1, v2) = m317;
                let m315 = v2;
                match m315.uncons() {
                    None => {
                        Err::<(bool, List<Adt0>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt0>), bool>(({
                            let c316: Fn9 = v0.clone();
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

pub fn f53(v0: bool, v1: List<Bytes>, v2: bool, v3: Adt0) -> bool {
    if v2 {
        true
    } else {
        f66(v3, v1, v0)
    }
}

pub fn f54(v0: List<Adt0>) -> List<Adt0> {
    f55(Fn10::F67, List::<Adt0>::nil(), v0)
}

pub fn f55(v0: Fn10, p323: List<Adt0>, p324: List<Adt0>) -> List<Adt0> {
    let m322 = (p323, p324);
    {
        let mut m322 = m322;
        loop {
            let step = {
                let (v1, v2) = m322;
                let m320 = v2;
                match m320.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c321: Fn10 = v0.clone();
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

pub fn f56(v0: bool, v1: List<Bytes>, v2: List<(Bytes, Adt1)>, v3: List<Adt0>, v4: Adt0) -> List<Adt0> {
    List::cons(f68(v4, v1, v0, v2), v3)
}

pub fn f57(v0: Bytes, v1: u64) -> Option<u64> {
    let a325 = 0u64;
    let a326 = v1;
    if nat_lt(a325, a326) {
        if f69(v0, {
            let a327 = v1;
            let a328 = 1u64;
            nat_sub(a327, a328)
        }) {
            Some({
                let a329 = v1;
                let a330 = 1u64;
                nat_sub(a329, a330)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f58(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a331 = v2;
    let a332 = v0;
    if nat_lt(a331, a332) {
        if f69(v1, v2) {
            Ok(Some({
                let a333 = v2;
                let a334 = 1u64;
                nat_add(a333, a334)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f59(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let a336 = match v2.clone() {
        Adt4::C0(_, _, h335) => {
            h335
        }
    };
    let a337 = 0u64;
    if equal(a336, a337) {
        let v3: u64 = f70(v0.clone(), match v2.clone() {
            Adt4::C0(h338, _, _) => {
                h338
            }
        })?;
        let a339 = v3;
        let a340 = v1;
        if nat_lt(a339, a340) {
            let m341 = f50(v0.clone(), v3);
            match m341 {
                None => {
                    Ok(None::<Adt4>)
                }
                Some(v4) => {
                    let a342 = v4;
                    let a343 = 34u8;
                    if equal(a342, a343) {
                        let m344 = f48(v0.clone(), v3)?;
                        match m344 {
                            None => {
                                Ok(Some(Adt4::C0(v3, match v2 {
                                    Adt4::C0(_, h359, _) => {
                                        h359
                                    }
                                }, 2u64)))
                            }
                            Some(v5) => {
                                let v6: u64 = f70(v0.clone(), {
                                    let (_, h345) = v5;
                                    h345
                                })?;
                                let a346 = v6;
                                let a347 = v1;
                                if nat_lt(a346, a347) {
                                    let m348 = f50(v0.clone(), v6);
                                    match m348 {
                                        None => {
                                            Ok(None::<Adt4>)
                                        }
                                        Some(v7) => {
                                            let a349 = v7;
                                            let a350 = 44u8;
                                            if equal(a349, a350) {
                                                Ok(Some(Adt4::C0(f70(v0, {
                                                    let a351 = v6;
                                                    let a352 = 1u64;
                                                    nat_add(a351, a352)?
                                                })?, List::cons({
                                                    let (h353, _) = v5;
                                                    h353
                                                }, match v2 {
                                                    Adt4::C0(_, h354, _) => {
                                                        h354
                                                    }
                                                }), 0u64)))
                                            } else {
                                                Ok(Some(Adt4::C0(v6, List::cons({
                                                    let (h355, _) = v5;
                                                    h355
                                                }, match v2 {
                                                    Adt4::C0(_, h356, _) => {
                                                        h356
                                                    }
                                                }), 2u64)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok(Some(Adt4::C0(v6, List::cons({
                                        let (h357, _) = v5;
                                        h357
                                    }, match v2 {
                                        Adt4::C0(_, h358, _) => {
                                            h358
                                        }
                                    }), 1u64)))
                                }
                            }
                        }
                    } else {
                        Ok(Some(Adt4::C0(v3, match v2 {
                            Adt4::C0(_, h360, _) => {
                                h360
                            }
                        }, 2u64)))
                    }
                }
            }
        } else {
            Ok(Some(Adt4::C0(v3, match v2 {
                Adt4::C0(_, h361, _) => {
                    h361
                }
            }, 1u64)))
        }
    } else {
        Ok(None::<Adt4>)
    }
}

pub fn f60(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f71(v1, v0, v2)
}

pub fn f61(v0: Fn11, p366: u64, p367: Adt5) -> R<(Adt5, bool)> {
    let m365 = (p366, p367);
    {
        let mut m365 = m365;
        loop {
            let step = {
                let (v1, v2) = m365;
                let m362 = v1;
                if m362 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m362 - 1;
                    let m364 = {
                        let c363: Fn11 = v0.clone();
                        c363.apply(v2.clone())?
                    };
                    match m364 {
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
                Ok(next) => { m365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    f72(v0, v1, v2)
}

pub fn f63(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a368 = v0.clone();
        length_bytes(a368)
    };
    let a369 = 0u64;
    let a370 = v1;
    if nat_lt(a369, a370) {
        let a371 = f73(v0.clone())?;
        let a372 = v1;
        if nat_eq(a371, a372) {
            let a373 = 1u64;
            let a374 = v1;
            if nat_lt(a373, a374) {
                let m377 = {
                    let a375 = v0;
                    let a376 = 0u64;
                    index_bytes(a375, a376)
                };
                match m377 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a378 = v2;
                        let a379 = 48u8;
                        Ok(!equal(a378, a379))
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

pub fn f64(v0: Fn12, p384: u64, p385: (u64, u64)) -> R<((u64, u64), bool)> {
    let m383 = (p384, p385);
    {
        let mut m383 = m383;
        loop {
            let step = {
                let (v1, v2) = m383;
                let m380 = v1;
                if m380 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m380 - 1;
                    let m382 = {
                        let c381: Fn12 = v0.clone();
                        c381.apply(v2)?
                    };
                    match m382 {
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
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f74(v0, v1)
}

pub fn f66(v0: Adt0, v1: List<Bytes>, v2: bool) -> bool {
    if f75(match v0.clone() {
        Adt0::C0(h386, _, _) => {
            h386
        }
    }, v1) {
        let a388 = match v0 {
            Adt0::C0(_, h387, _) => {
                h387
            }
        };
        let a389 = v2;
        equal(a388, a389)
    } else {
        false
    }
}

pub fn f67(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f68(v0: Adt0, v1: List<Bytes>, v2: bool, v3: List<(Bytes, Adt1)>) -> Adt0 {
    if f66(v0.clone(), v1, v2) {
        Adt0::C0(match v0.clone() {
            Adt0::C0(h390, _, _) => {
                h390
            }
        }, match v0.clone() {
            Adt0::C0(_, h391, _) => {
                h391
            }
        }, f76(List::cons(v3, f76(match v0 {
            Adt0::C0(_, _, h392) => {
                h392
            }
        }))))
    } else {
        v0
    }
}

pub fn f69(v0: Bytes, v1: u64) -> bool {
    let m393 = f50(v0, v1);
    match m393 {
        None => {
            false
        }
        Some(v2) => {
            f77(v2)
        }
    }
}

pub fn f70(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a394 = v0.clone();
        length_bytes(a394)
    };
    let v3: u64 = {
        let a395 = v2;
        let a396 = v1;
        nat_sub(a395, a396)
    };
    let a397 = v1;
    let a398 = f38(f12(v0, v1, v3), v3)?;
    nat_add(a397, a398)
}

pub fn f71(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h399, _) = v2.clone();
        h399
    };
    let v4: u64 = {
        let (h400, _) = {
            let (_, h401) = v2.clone();
            h401
        };
        h400
    };
    let v5: List<Bytes> = {
        let (_, h402) = {
            let (_, h403) = v2;
            h403
        };
        h402
    };
    let m404 = f50(v0.clone(), v3);
    match m404 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a405 = v6;
            let a406 = v1;
            if equal(a405, a406) {
                Ok(Some(({
                    let a407 = v3;
                    let a408 = 1u64;
                    nat_add(a407, a408)?
                }, ({
                    let a409 = v3;
                    let a410 = 1u64;
                    nat_add(a409, a410)?
                }, List::cons(f12(v0, v4, {
                    let a411 = v3;
                    let a412 = v4;
                    nat_sub(a411, a412)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a413 = v3;
                    let a414 = 1u64;
                    nat_add(a413, a414)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    if match v2.clone() {
        Adt5::C0(_, _, _, h415) => {
            h415
        }
    } {
        Ok(None::<Adt5>)
    } else {
        let v3: u64 = match v2.clone() {
            Adt5::C0(h416, _, _, _) => {
                h416
            }
        };
        let a417 = v3;
        let a418 = v1;
        if nat_lt(a417, a418) {
            let v4: u64 = match v2.clone() {
                Adt5::C0(_, _, h419, _) => {
                    h419
                }
            };
            let a420 = v4;
            let a421 = 1u64;
            if nat_eq(a420, a421) {
                let m422 = f50(v0, v3);
                match m422 {
                    None => {
                        Ok(None::<Adt5>)
                    }
                    Some(v5) => {
                        let a423 = v5;
                        let a424 = 34u8;
                        if if equal(a423, a424) {
                            true
                        } else {
                            let a425 = v5;
                            let a426 = 92u8;
                            if equal(a425, a426) {
                                true
                            } else {
                                let a427 = v5;
                                let a428 = 110u8;
                                if equal(a427, a428) {
                                    true
                                } else {
                                    let a429 = v5;
                                    let a430 = 116u8;
                                    if equal(a429, a430) {
                                        true
                                    } else {
                                        let a431 = v5;
                                        let a432 = 114u8;
                                        equal(a431, a432)
                                    }
                                }
                            }
                        } {
                            Ok(Some(Adt5::C0({
                                let a433 = v3;
                                let a434 = 1u64;
                                nat_add(a433, a434)?
                            }, f78(match v2 {
                                Adt5::C0(_, h435, _, _) => {
                                    h435
                                }
                            }, {
                                let a436 = v5;
                                let a437 = 34u8;
                                if equal(a436, a437) {
                                    Bytes::lit(&[0x22])
                                } else {
                                    let a438 = v5;
                                    let a439 = 92u8;
                                    if equal(a438, a439) {
                                        Bytes::lit(&[0x5c])
                                    } else {
                                        let a440 = v5;
                                        let a441 = 110u8;
                                        if equal(a440, a441) {
                                            Bytes::lit(&[0x0a])
                                        } else {
                                            let a442 = v5;
                                            let a443 = 116u8;
                                            if equal(a442, a443) {
                                                Bytes::lit(&[0x09])
                                            } else {
                                                Bytes::lit(&[0x0d])
                                            }
                                        }
                                    }
                                }
                            }), 0u64, false)))
                        } else {
                            Ok(None::<Adt5>)
                        }
                    }
                }
            } else {
                let m444 = f50(v0.clone(), v3);
                match m444 {
                    None => {
                        Ok(None::<Adt5>)
                    }
                    Some(v6) => {
                        let a445 = v6;
                        let a446 = 34u8;
                        if equal(a445, a446) {
                            Ok(Some(Adt5::C0({
                                let a447 = v3;
                                let a448 = 1u64;
                                nat_add(a447, a448)?
                            }, match v2 {
                                Adt5::C0(_, h449, _, _) => {
                                    h449
                                }
                            }, 0u64, true)))
                        } else {
                            let a450 = v6;
                            let a451 = 92u8;
                            if equal(a450, a451) {
                                Ok(Some(Adt5::C0({
                                    let a452 = v3;
                                    let a453 = 1u64;
                                    nat_add(a452, a453)?
                                }, match v2 {
                                    Adt5::C0(_, h454, _, _) => {
                                        h454
                                    }
                                }, 1u64, false)))
                            } else {
                                Ok(Some(Adt5::C0({
                                    let a455 = v3;
                                    let a456 = 1u64;
                                    nat_add(a455, a456)?
                                }, f78(match v2 {
                                    Adt5::C0(_, h457, _, _) => {
                                        h457
                                    }
                                }, f12(v0, v3, 1u64)), 0u64, false)))
                            }
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f73(v0: Bytes) -> R<u64> {
    let (h458, _) = f43(Fn7::F79(v0.clone()), {
        let a459 = v0;
        length_bytes(a459)
    }, 0u64)?;
    Ok(h458)
}

pub fn f74(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m463 = {
        let a461 = v0;
        let a462 = {
            let (h460, _) = v1;
            h460
        };
        index_bytes(a461, a462)
    };
    match m463 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a465 = {
                    let (h464, _) = v1;
                    h464
                };
                let a466 = 1u64;
                nat_add(a465, a466)?
            }, {
                let a470 = {
                    let a468 = {
                        let (_, h467) = v1;
                        h467
                    };
                    let a469 = 10u64;
                    nat_mul(a468, a469)?
                };
                let a471 = f80(v2)?;
                nat_add(a470, a471)?
            })))
        }
    }
}

pub fn f75(v0: List<Bytes>, v1: List<Bytes>) -> bool {
    let a472 = f81(v0, Bytes::lit(&[0x2e]));
    let a473 = f81(v1, Bytes::lit(&[0x2e]));
    equal(a472, a473)
}

pub fn f76(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f82(Fn13::F83, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f77(v0: u8) -> bool {
    let a474 = v0;
    let a475 = 32u8;
    if if if equal(a474, a475) {
        true
    } else {
        let a476 = v0;
        let a477 = 9u8;
        equal(a476, a477)
    } {
        true
    } else {
        let a478 = v0;
        let a479 = 10u8;
        equal(a478, a479)
    } {
        true
    } else {
        let a480 = v0;
        let a481 = 12u8;
        if equal(a480, a481) {
            true
        } else {
            let a482 = v0;
            let a483 = 13u8;
            equal(a482, a483)
        }
    }
}

pub fn f78(v0: Bytes, v1: Bytes) -> Bytes {
    let a484 = v0;
    let a485 = v1;
    append_bytes(a484, a485)
}

pub fn f79(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m488 = {
        let a486 = v0;
        let a487 = v1;
        index_bytes(a486, a487)
    };
    match m488 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f84(v2)? {
                Ok(Some({
                    let a489 = v1;
                    let a490 = 1u64;
                    nat_add(a489, a490)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f80(v0: u8) -> R<u64> {
    let a537 = {
        let v1: u8 = v0;
        let a535 = {
            let a511 = {
                let a499 = {
                    let a493 = {
                        let a491 = v1;
                        let a492 = 128u8;
                        fixed_u8::bit_and(a491, a492)
                    };
                    let a494 = 128u8;
                    if equal(a493, a494) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a500 = {
                    let a497 = {
                        let a495 = v1;
                        let a496 = 64u8;
                        fixed_u8::bit_and(a495, a496)
                    };
                    let a498 = 64u8;
                    if equal(a497, a498) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a499, a500)?
            };
            let a512 = {
                let a509 = {
                    let a503 = {
                        let a501 = v1;
                        let a502 = 32u8;
                        fixed_u8::bit_and(a501, a502)
                    };
                    let a504 = 32u8;
                    if equal(a503, a504) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a510 = {
                    let a507 = {
                        let a505 = v1;
                        let a506 = 16u8;
                        fixed_u8::bit_and(a505, a506)
                    };
                    let a508 = 16u8;
                    if equal(a507, a508) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a509, a510)?
            };
            nat_add(a511, a512)?
        };
        let a536 = {
            let a533 = {
                let a521 = {
                    let a515 = {
                        let a513 = v1;
                        let a514 = 8u8;
                        fixed_u8::bit_and(a513, a514)
                    };
                    let a516 = 8u8;
                    if equal(a515, a516) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a522 = {
                    let a519 = {
                        let a517 = v1;
                        let a518 = 4u8;
                        fixed_u8::bit_and(a517, a518)
                    };
                    let a520 = 4u8;
                    if equal(a519, a520) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a521, a522)?
            };
            let a534 = {
                let a531 = {
                    let a525 = {
                        let a523 = v1;
                        let a524 = 2u8;
                        fixed_u8::bit_and(a523, a524)
                    };
                    let a526 = 2u8;
                    if equal(a525, a526) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a532 = {
                    let a529 = {
                        let a527 = v1;
                        let a528 = 1u8;
                        fixed_u8::bit_and(a527, a528)
                    };
                    let a530 = 1u8;
                    if equal(a529, a530) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a531, a532)?
            };
            nat_add(a533, a534)?
        };
        nat_add(a535, a536)?
    };
    let a538 = 48u64;
    Ok(nat_sub(a537, a538))
}

pub fn f81(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m539 = f85(Fn14::F86(v1), None::<Bytes>, v0);
    match m539 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f82(v0: Fn13, p543: List<List<(Bytes, Adt1)>>, p544: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m542 = (p543, p544);
    {
        let mut m542 = m542;
        loop {
            let step = {
                let (v1, v2) = m542;
                let m540 = v2;
                match m540.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c541: Fn13 = v0.clone();
                            c541.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f84(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a589 = {
            let a565 = {
                let a553 = {
                    let a547 = {
                        let a545 = v1;
                        let a546 = 128u8;
                        fixed_u8::bit_and(a545, a546)
                    };
                    let a548 = 128u8;
                    if equal(a547, a548) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a554 = {
                    let a551 = {
                        let a549 = v1;
                        let a550 = 64u8;
                        fixed_u8::bit_and(a549, a550)
                    };
                    let a552 = 64u8;
                    if equal(a551, a552) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a553, a554)?
            };
            let a566 = {
                let a563 = {
                    let a557 = {
                        let a555 = v1;
                        let a556 = 32u8;
                        fixed_u8::bit_and(a555, a556)
                    };
                    let a558 = 32u8;
                    if equal(a557, a558) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a564 = {
                    let a561 = {
                        let a559 = v1;
                        let a560 = 16u8;
                        fixed_u8::bit_and(a559, a560)
                    };
                    let a562 = 16u8;
                    if equal(a561, a562) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a563, a564)?
            };
            nat_add(a565, a566)?
        };
        let a590 = {
            let a587 = {
                let a575 = {
                    let a569 = {
                        let a567 = v1;
                        let a568 = 8u8;
                        fixed_u8::bit_and(a567, a568)
                    };
                    let a570 = 8u8;
                    if equal(a569, a570) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a576 = {
                    let a573 = {
                        let a571 = v1;
                        let a572 = 4u8;
                        fixed_u8::bit_and(a571, a572)
                    };
                    let a574 = 4u8;
                    if equal(a573, a574) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a575, a576)?
            };
            let a588 = {
                let a585 = {
                    let a579 = {
                        let a577 = v1;
                        let a578 = 2u8;
                        fixed_u8::bit_and(a577, a578)
                    };
                    let a580 = 2u8;
                    if equal(a579, a580) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a586 = {
                    let a583 = {
                        let a581 = v1;
                        let a582 = 1u8;
                        fixed_u8::bit_and(a581, a582)
                    };
                    let a584 = 1u8;
                    if equal(a583, a584) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a585, a586)?
            };
            nat_add(a587, a588)?
        };
        nat_add(a589, a590)?
    };
    let a591 = 48u64;
    let a592 = v2;
    if nat_le(a591, a592) {
        let a593 = v2;
        let a594 = 57u64;
        Ok(nat_le(a593, a594))
    } else {
        Ok(false)
    }
}

pub fn f85(v0: Fn14, p598: Option<Bytes>, p599: List<Bytes>) -> Option<Bytes> {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v1, v2) = m597;
                let m595 = v2;
                match m595.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c596: Fn14 = v0.clone();
                            c596.apply(v1, v3)
                        }, v4))
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

pub fn f86(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m600 = v1;
    match m600 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a603 = {
                    let a601 = v3;
                    let a602 = v0;
                    append_bytes(a601, a602)
                };
                let a604 = v2;
                append_bytes(a603, a604)
            })
        }
    }
}

pub fn run(p0: Bytes) -> R<Option<List<Adt0>>> {
    f0(p0)
}
