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
    C0(Adt0, u64, List<Adt0>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F5,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
        match self {
            Fn0::F5 => f5(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7(Bytes, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn1::F7(k0, k1) => f7(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt2, Adt1)) -> (Adt2, Adt1) {
        match self {
            Fn2::F17 => f17(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F19(u64, Bytes, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn3::F19(k0, k1, k2) => f19(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F23,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F26,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt3) -> Adt3 {
        match self {
            Fn5::F26 => f26(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Adt0> {
    f1(v0)
}

pub fn f1(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: u64 = {
        let a2 = v1;
        let a3 = 0u64;
        if nat_eq(a2, a3) {
            1u64
        } else {
            let a6 = {
                let a4 = v1;
                let a5 = 1023u64;
                nat_add(a4, a5)?
            };
            let a7 = 1024u64;
            let a8 = 0u64;
            nat_quot(a6, a7, a8)
        }
    };
    let v3: u64 = {
        let a9 = v2;
        let a10 = 1u64;
        nat_sub(a9, a10)
    };
    let m12 = {
        let (_, h11) = f2(v0.clone(), v1, v3)?;
        h11
    };
    match m12.uncons() {
        None => {
            f3(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h13) = f4(Fn0::F5, 64u64, (List::cons(v4, v5), f3(v0, v1, v3, false)?));
            Ok(h13)
        }
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt0>)> {
    f6(Fn1::F7(v0, v1), v2, (0u64, List::<Adt0>::nil()))
}

pub fn f3(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt0> {
    let v4: u64 = {
        let a14 = 1024u64;
        let a15 = v2;
        nat_mul(a14, a15)?
    };
    let v5: u64 = f8(v1, v4);
    let v6: u64 = f9(v5)?;
    let v7: u64 = {
        let a16 = v6;
        let a17 = 1u64;
        nat_sub(a16, a17)
    };
    Ok(f10(f11(v0.clone(), v4, v2, v7)?, f12(v0, {
        let a20 = v4;
        let a21 = {
            let a18 = 64u64;
            let a19 = v7;
            nat_mul(a18, a19)?
        };
        nat_add(a20, a21)?
    })?, v2, f13({
        let a24 = v5;
        let a25 = {
            let a22 = 64u64;
            let a23 = v7;
            nat_mul(a22, a23)?
        };
        nat_sub(a24, a25)
    }), {
        let a28 = {
            let a26 = v7;
            let a27 = 0u64;
            if nat_eq(a26, a27) {
                3u32
            } else {
                2u32
            }
        };
        let a29 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a28, a29)
    }))
}

pub fn f4(v0: Fn0, p33: u64, p34: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m30 = v1;
                if m30 == 0 {
                    Err::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>(v2)
                } else {
                    let v3 = m30 - 1;
                    Ok::<(u64, (List<Adt0>, Adt0)), (List<Adt0>, Adt0)>((v3, {
                        let c31: Fn0 = v0.clone();
                        c31.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    f14(v0)
}

pub fn f6(v0: Fn1, p38: u64, p39: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v1, v2) = m37;
                let m35 = v1;
                if m35 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m35 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c36: Fn1 = v0.clone();
                        c36.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    f15(v2.clone(), f3(v0, v1, {
        let (h40, _) = v2;
        h40
    }, false)?)
}

pub fn f8(v0: u64, v1: u64) -> u64 {
    let a43 = 1024u64;
    let a44 = {
        let a41 = v0;
        let a42 = v1;
        nat_sub(a41, a42)
    };
    if nat_lt(a43, a44) {
        1024u64
    } else {
        let a45 = v0;
        let a46 = v1;
        nat_sub(a45, a46)
    }
}

pub fn f9(v0: u64) -> R<u64> {
    let a47 = v0;
    let a48 = 0u64;
    if nat_eq(a47, a48) {
        Ok(1u64)
    } else {
        let a51 = {
            let a49 = v0;
            let a50 = 63u64;
            nat_add(a49, a50)?
        };
        let a52 = 64u64;
        let a53 = 0u64;
        Ok(nat_quot(a51, a52, a53))
    }
}

pub fn f10(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt2 = {
        let (h54, _) = f16(Fn2::F17, 7u64, (Adt2::C0(match v0.clone() {
            Adt0::C0(h55, _, _, _, _, _, _, _) => {
                h55
            }
        }, match v0.clone() {
            Adt0::C0(_, h56, _, _, _, _, _, _) => {
                h56
            }
        }, match v0.clone() {
            Adt0::C0(_, _, h57, _, _, _, _, _) => {
                h57
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, h58, _, _, _, _) => {
                h58
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, h59, _, _, _) => {
                h59
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, h60, _, _) => {
                h60
            }
        }, match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h61, _) => {
                h61
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h62) => {
                h62
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f13({
            let a63 = v2;
            let a64 = 4294967296u64;
            let a65 = 0u64;
            nat_rem(a63, a64, a65)
        }), f13({
            let a66 = v2;
            let a67 = 4294967296u64;
            let a68 = 0u64;
            nat_quot(a66, a67, a68)
        }), v3, v4), v1));
        h54
    };
    Adt0::C0({
        let a71 = match v5.clone() {
            Adt2::C0(h69, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        };
        let a72 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h70, _, _, _, _, _, _, _) => {
                h70
            }
        };
        fixed_u32::bit_xor(a71, a72)
    }, {
        let a75 = match v5.clone() {
            Adt2::C0(_, h73, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a76 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h74, _, _, _, _, _, _) => {
                h74
            }
        };
        fixed_u32::bit_xor(a75, a76)
    }, {
        let a79 = match v5.clone() {
            Adt2::C0(_, _, h77, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        };
        let a80 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h78, _, _, _, _, _) => {
                h78
            }
        };
        fixed_u32::bit_xor(a79, a80)
    }, {
        let a83 = match v5.clone() {
            Adt2::C0(_, _, _, h81, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h81
            }
        };
        let a84 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h82, _, _, _, _) => {
                h82
            }
        };
        fixed_u32::bit_xor(a83, a84)
    }, {
        let a87 = match v5.clone() {
            Adt2::C0(_, _, _, _, h85, _, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        };
        let a88 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h86, _, _, _) => {
                h86
            }
        };
        fixed_u32::bit_xor(a87, a88)
    }, {
        let a91 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, h89, _, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        };
        let a92 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h90, _, _) => {
                h90
            }
        };
        fixed_u32::bit_xor(a91, a92)
    }, {
        let a95 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, h93, _, _, _, _, _, _, _, _, _) => {
                h93
            }
        };
        let a96 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h94, _) => {
                h94
            }
        };
        fixed_u32::bit_xor(a95, a96)
    }, {
        let a99 = match v5.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h97, _, _, _, _, _, _, _, _) => {
                h97
            }
        };
        let a100 = match v5 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h98) => {
                h98
            }
        };
        fixed_u32::bit_xor(a99, a100)
    })
}

pub fn f11(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (_, h101) = f18(Fn3::F19(v2, v0, v1), v3, (0u64, f20()))?;
    Ok(h101)
}

pub fn f12(v0: Bytes, v1: u64) -> R<Adt1> {
    Ok(Adt1::C0(f21(v0.clone(), {
        let a102 = v1;
        let a103 = 0u64;
        nat_add(a102, a103)?
    })?, f21(v0.clone(), {
        let a104 = v1;
        let a105 = 4u64;
        nat_add(a104, a105)?
    })?, f21(v0.clone(), {
        let a106 = v1;
        let a107 = 8u64;
        nat_add(a106, a107)?
    })?, f21(v0.clone(), {
        let a108 = v1;
        let a109 = 12u64;
        nat_add(a108, a109)?
    })?, f21(v0.clone(), {
        let a110 = v1;
        let a111 = 16u64;
        nat_add(a110, a111)?
    })?, f21(v0.clone(), {
        let a112 = v1;
        let a113 = 20u64;
        nat_add(a112, a113)?
    })?, f21(v0.clone(), {
        let a114 = v1;
        let a115 = 24u64;
        nat_add(a114, a115)?
    })?, f21(v0.clone(), {
        let a116 = v1;
        let a117 = 28u64;
        nat_add(a116, a117)?
    })?, f21(v0.clone(), {
        let a118 = v1;
        let a119 = 32u64;
        nat_add(a118, a119)?
    })?, f21(v0.clone(), {
        let a120 = v1;
        let a121 = 36u64;
        nat_add(a120, a121)?
    })?, f21(v0.clone(), {
        let a122 = v1;
        let a123 = 40u64;
        nat_add(a122, a123)?
    })?, f21(v0.clone(), {
        let a124 = v1;
        let a125 = 44u64;
        nat_add(a124, a125)?
    })?, f21(v0.clone(), {
        let a126 = v1;
        let a127 = 48u64;
        nat_add(a126, a127)?
    })?, f21(v0.clone(), {
        let a128 = v1;
        let a129 = 52u64;
        nat_add(a128, a129)?
    })?, f21(v0.clone(), {
        let a130 = v1;
        let a131 = 56u64;
        nat_add(a130, a131)?
    })?, f21(v0, {
        let a132 = v1;
        let a133 = 60u64;
        nat_add(a132, a133)?
    })?))
}

pub fn f13(v0: u64) -> u32 {
    let (_, h134) = {
        let (_, h135) = f22(Fn4::F23, 32u64, (v0, (1u32, 0u32)));
        h135
    };
    h134
}

pub fn f14(v0: (List<Adt0>, Adt0)) -> (List<Adt0>, Adt0) {
    let m137 = {
        let (h136, _) = v0.clone();
        h136
    };
    match m137.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f24(v1, {
                let (_, h138) = v0;
                h138
            }, {
                let m139 = v2;
                match m139.uncons() {
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

pub fn f15(v0: (u64, List<Adt0>), v1: Adt0) -> R<(u64, List<Adt0>)> {
    let v2: Adt3 = f25(Fn5::F26, 40u64, Adt3::C0(v1, {
        let a141 = {
            let (h140, _) = v0.clone();
            h140
        };
        let a142 = 1u64;
        nat_add(a141, a142)?
    }, {
        let (_, h143) = v0;
        h143
    }));
    Ok(({
        let a145 = {
            let (h144, _) = v0;
            h144
        };
        let a146 = 1u64;
        nat_add(a145, a146)?
    }, List::cons(match v2.clone() {
        Adt3::C0(h147, _, _) => {
            h147
        }
    }, match v2 {
        Adt3::C0(_, _, h148) => {
            h148
        }
    })))
}

pub fn f16(v0: Fn2, p152: u64, p153: (Adt2, Adt1)) -> (Adt2, Adt1) {
    let m151 = (p152, p153);
    {
        let mut m151 = m151;
        loop {
            let step = {
                let (v1, v2) = m151;
                let m149 = v1;
                if m149 == 0 {
                    Err::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>(v2)
                } else {
                    let v3 = m149 - 1;
                    Ok::<(u64, (Adt2, Adt1)), (Adt2, Adt1)>((v3, {
                        let c150: Fn2 = v0.clone();
                        c150.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    f27(v0)
}

pub fn f18(v0: Fn3, p157: u64, p158: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v1, v2) = m156;
                let m154 = v1;
                if m154 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m154 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c155: Fn3 = v0.clone();
                        c155.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a160 = {
            let (h159, _) = v3.clone();
            h159
        };
        let a161 = 1u64;
        nat_add(a160, a161)?
    }, f10({
        let (_, h162) = v3.clone();
        h162
    }, f12(v1, {
        let a166 = v2;
        let a167 = {
            let a164 = 64u64;
            let a165 = {
                let (h163, _) = v3.clone();
                h163
            };
            nat_mul(a164, a165)?
        };
        nat_add(a166, a167)?
    })?, v0, 64u32, {
        let a169 = {
            let (h168, _) = v3;
            h168
        };
        let a170 = 0u64;
        if nat_eq(a169, a170) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f20() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Bytes, v1: u64) -> R<u32> {
    let a181 = f28(v0.clone(), v1);
    let a182 = {
        let a179 = f29(f28(v0.clone(), {
            let a171 = v1;
            let a172 = 1u64;
            nat_add(a171, a172)?
        }), 8u32);
        let a180 = {
            let a177 = f29(f28(v0.clone(), {
                let a173 = v1;
                let a174 = 2u64;
                nat_add(a173, a174)?
            }), 16u32);
            let a178 = f29(f28(v0, {
                let a175 = v1;
                let a176 = 3u64;
                nat_add(a175, a176)?
            }), 24u32);
            fixed_u32::bit_or(a177, a178)
        };
        fixed_u32::bit_or(a179, a180)
    };
    Ok(fixed_u32::bit_or(a181, a182))
}

pub fn f22(v0: Fn4, p186: u64, p187: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m185 = (p186, p187);
    {
        let mut m185 = m185;
        loop {
            let step = {
                let (v1, v2) = m185;
                let m183 = v1;
                if m183 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m183 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c184: Fn4 = v0.clone();
                        c184.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h188, _) = v0;
        h188
    };
    let v2: u32 = {
        let (h189, _) = {
            let (_, h190) = v0;
            h190
        };
        h189
    };
    let v3: u32 = {
        let (_, h191) = {
            let (_, h192) = v0;
            h192
        };
        h191
    };
    ({
        let a193 = v1;
        let a194 = 2u64;
        let a195 = 0u64;
        nat_quot(a193, a194, a195)
    }, (f29(v2, 1u32), {
        let a199 = {
            let a196 = v1;
            let a197 = 2u64;
            let a198 = 0u64;
            nat_rem(a196, a197, a198)
        };
        let a200 = 1u64;
        if nat_eq(a199, a200) {
            let a201 = v3;
            let a202 = v2;
            fixed_u32::bit_or(a201, a202)
        } else {
            v3
        }
    }))
}

pub fn f24(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f10(f20(), f30(v0, v1), 0u64, 64u32, {
        let a203 = 4u32;
        let a204 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a203, a204)
    })
}

pub fn f25(v0: Fn5, p208: u64, p209: Adt3) -> Adt3 {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m205 = v1;
                if m205 == 0 {
                    Err::<(u64, Adt3), Adt3>(v2)
                } else {
                    let v3 = m205 - 1;
                    Ok::<(u64, Adt3), Adt3>((v3, {
                        let c206: Fn5 = v0.clone();
                        c206.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt3) -> Adt3 {
    f31(v0)
}

pub fn f27(v0: (Adt2, Adt1)) -> (Adt2, Adt1) {
    (f32(f33({
        let (h210, _) = v0.clone();
        h210
    }, {
        let (_, h211) = v0.clone();
        h211
    }), {
        let (_, h212) = v0.clone();
        h212
    }), f34({
        let (_, h213) = v0;
        h213
    }))
}

pub fn f28(v0: Bytes, v1: u64) -> u32 {
    let m216 = {
        let a214 = v0;
        let a215 = v1;
        index_bytes(a214, a215)
    };
    match m216 {
        None => {
            0u32
        }
        Some(v2) => {
            let m218 = {
                let a217 = v2;
                fixed_u32::convert(i128::from(a217))
            };
            match m218 {
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

pub fn f29(v0: u32, v1: u32) -> u32 {
    let m221 = {
        let a219 = v0;
        let a220 = v1;
        fixed_u32::shift_left(a219, a220)
    };
    match m221 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: Adt0, v1: Adt0) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt0::C0(h222, _, _, _, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt0::C0(_, h223, _, _, _, _, _, _) => {
            h223
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h224, _, _, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h225, _, _, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h226, _, _, _) => {
            h226
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h227, _, _) => {
            h227
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h228, _) => {
            h228
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h229) => {
            h229
        }
    }, match v1.clone() {
        Adt0::C0(h230, _, _, _, _, _, _, _) => {
            h230
        }
    }, match v1.clone() {
        Adt0::C0(_, h231, _, _, _, _, _, _) => {
            h231
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h232, _, _, _, _, _) => {
            h232
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h233, _, _, _, _) => {
            h233
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h234, _, _, _) => {
            h234
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h235, _, _) => {
            h235
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h236, _) => {
            h236
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h237) => {
            h237
        }
    })
}

pub fn f31(v0: Adt3) -> Adt3 {
    let a242 = {
        let a239 = match v0.clone() {
            Adt3::C0(_, h238, _) => {
                h238
            }
        };
        let a240 = 2u64;
        let a241 = 0u64;
        nat_rem(a239, a240, a241)
    };
    let a243 = 0u64;
    if nat_eq(a242, a243) {
        let m245 = match v0.clone() {
            Adt3::C0(_, _, h244) => {
                h244
            }
        };
        match m245.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt3::C0(f24(v1, match v0.clone() {
                    Adt3::C0(h246, _, _) => {
                        h246
                    }
                }, false), {
                    let a248 = match v0 {
                        Adt3::C0(_, h247, _) => {
                            h247
                        }
                    };
                    let a249 = 2u64;
                    let a250 = 0u64;
                    nat_quot(a248, a249, a250)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f32(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f35(match v0.clone() {
        Adt2::C0(h251, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h251
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h252, _, _, _, _, _, _, _, _, _, _) => {
            h252
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h253, _, _, _, _, _) => {
            h253
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h254) => {
            h254
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h255, _, _, _, _, _, _, _) => {
            h255
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h256, _, _, _, _, _, _) => {
            h256
        }
    });
    let v3: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, h257, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h257
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h258, _, _, _, _, _, _, _, _, _) => {
            h258
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h259, _, _, _, _) => {
            h259
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h260, _, _, _) => {
            h260
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h261, _, _, _, _, _) => {
            h261
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h262, _, _, _, _) => {
            h262
        }
    });
    let v4: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, h263, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h263
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h264, _, _, _, _, _, _, _, _) => {
            h264
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h265, _, _, _, _, _, _, _) => {
            h265
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h266, _, _) => {
            h266
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h267, _, _, _) => {
            h267
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h268, _, _) => {
            h268
        }
    });
    let v5: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, _, h269, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h270, _, _, _, _, _, _, _, _, _, _, _) => {
            h270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h271, _, _, _, _, _, _) => {
            h271
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h272, _) => {
            h272
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h273, _) => {
            h273
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h274) => {
            h274
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h275, _, _, _) => {
            h275
        }
    }, match v3.clone() {
        Adt4::C0(h276, _, _, _) => {
            h276
        }
    }, match v4.clone() {
        Adt4::C0(h277, _, _, _) => {
            h277
        }
    }, match v5.clone() {
        Adt4::C0(h278, _, _, _) => {
            h278
        }
    }, match v5.clone() {
        Adt4::C0(_, h279, _, _) => {
            h279
        }
    }, match v2.clone() {
        Adt4::C0(_, h280, _, _) => {
            h280
        }
    }, match v3.clone() {
        Adt4::C0(_, h281, _, _) => {
            h281
        }
    }, match v4.clone() {
        Adt4::C0(_, h282, _, _) => {
            h282
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h283, _) => {
            h283
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h284, _) => {
            h284
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h285, _) => {
            h285
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h286, _) => {
            h286
        }
    }, match v3 {
        Adt4::C0(_, _, _, h287) => {
            h287
        }
    }, match v4 {
        Adt4::C0(_, _, _, h288) => {
            h288
        }
    }, match v5 {
        Adt4::C0(_, _, _, h289) => {
            h289
        }
    }, match v2 {
        Adt4::C0(_, _, _, h290) => {
            h290
        }
    })
}

pub fn f33(v0: Adt2, v1: Adt1) -> Adt2 {
    let v2: Adt4 = f35(match v0.clone() {
        Adt2::C0(h291, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h291
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h292, _, _, _, _, _, _, _, _, _, _, _) => {
            h292
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h293, _, _, _, _, _, _, _) => {
            h293
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, h294, _, _, _) => {
            h294
        }
    }, match v1.clone() {
        Adt1::C0(h295, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h295
        }
    }, match v1.clone() {
        Adt1::C0(_, h296, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h296
        }
    });
    let v3: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, h297, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h297
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h298, _, _, _, _, _, _, _, _, _, _) => {
            h298
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h299, _, _, _, _, _, _) => {
            h299
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h300, _, _) => {
            h300
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h301, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h301
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h302, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h302
        }
    });
    let v4: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, h303, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h303
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h304, _, _, _, _, _, _, _, _, _) => {
            h304
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h305, _, _, _, _, _) => {
            h305
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h306, _) => {
            h306
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h307, _, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h308, _, _, _, _, _, _, _, _, _, _) => {
            h308
        }
    });
    let v5: Adt4 = f35(match v0.clone() {
        Adt2::C0(_, _, _, h309, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h310, _, _, _, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h311, _, _, _, _) => {
            h311
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h312) => {
            h312
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h313, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h314, _, _, _, _, _, _, _, _) => {
            h314
        }
    });
    Adt2::C0(match v2.clone() {
        Adt4::C0(h315, _, _, _) => {
            h315
        }
    }, match v3.clone() {
        Adt4::C0(h316, _, _, _) => {
            h316
        }
    }, match v4.clone() {
        Adt4::C0(h317, _, _, _) => {
            h317
        }
    }, match v5.clone() {
        Adt4::C0(h318, _, _, _) => {
            h318
        }
    }, match v2.clone() {
        Adt4::C0(_, h319, _, _) => {
            h319
        }
    }, match v3.clone() {
        Adt4::C0(_, h320, _, _) => {
            h320
        }
    }, match v4.clone() {
        Adt4::C0(_, h321, _, _) => {
            h321
        }
    }, match v5.clone() {
        Adt4::C0(_, h322, _, _) => {
            h322
        }
    }, match v2.clone() {
        Adt4::C0(_, _, h323, _) => {
            h323
        }
    }, match v3.clone() {
        Adt4::C0(_, _, h324, _) => {
            h324
        }
    }, match v4.clone() {
        Adt4::C0(_, _, h325, _) => {
            h325
        }
    }, match v5.clone() {
        Adt4::C0(_, _, h326, _) => {
            h326
        }
    }, match v2 {
        Adt4::C0(_, _, _, h327) => {
            h327
        }
    }, match v3 {
        Adt4::C0(_, _, _, h328) => {
            h328
        }
    }, match v4 {
        Adt4::C0(_, _, _, h329) => {
            h329
        }
    }, match v5 {
        Adt4::C0(_, _, _, h330) => {
            h330
        }
    })
}

pub fn f34(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h331
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h332, _, _, _, _, _, _, _, _, _) => {
            h332
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h333, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h333
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h334, _, _, _, _, _) => {
            h334
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h335, _, _, _, _, _, _, _, _) => {
            h335
        }
    }, match v0.clone() {
        Adt1::C0(h336, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h336
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h337, _, _, _, _, _, _, _, _, _, _, _) => {
            h337
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h338, _, _) => {
            h338
        }
    }, match v0.clone() {
        Adt1::C0(_, h339, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h339
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h340, _, _, _, _) => {
            h340
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h341, _, _, _) => {
            h341
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h342, _, _, _, _, _, _, _, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h343, _, _, _, _, _, _) => {
            h343
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h344, _) => {
            h344
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h345) => {
            h345
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h346, _, _, _, _, _, _, _) => {
            h346
        }
    })
}

pub fn f35(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt4 {
    let v6: u32 = f36(f36(v0, v1), v4);
    let v7: u32 = f37({
        let a347 = v3;
        let a348 = v6;
        fixed_u32::bit_xor(a347, a348)
    }, 16u32, 16u32);
    let v8: u32 = f36(v2, v7);
    let v9: u32 = f37({
        let a349 = v1;
        let a350 = v8;
        fixed_u32::bit_xor(a349, a350)
    }, 12u32, 20u32);
    let v10: u32 = f36(f36(v6, v9), v5);
    let v11: u32 = f37({
        let a351 = v7;
        let a352 = v10;
        fixed_u32::bit_xor(a351, a352)
    }, 8u32, 24u32);
    let v12: u32 = f36(v8, v11);
    let v13: u32 = f37({
        let a353 = v9;
        let a354 = v12;
        fixed_u32::bit_xor(a353, a354)
    }, 7u32, 25u32);
    Adt4::C0(v10, v13, v12, v11)
}

pub fn f36(v0: u32, v1: u32) -> u32 {
    let m357 = {
        let a355 = f38(v0);
        let a356 = f38(v1);
        fixed_u64::checked_add(a355, a356)
    };
    match m357 {
        None => {
            0u32
        }
        Some(v2) => {
            f39(v2)
        }
    }
}

pub fn f37(v0: u32, v1: u32, v2: u32) -> u32 {
    let a358 = f40(v0, v1);
    let a359 = f29(v0, v2);
    fixed_u32::bit_or(a358, a359)
}

pub fn f38(v0: u32) -> u64 {
    let m361 = {
        let a360 = v0;
        fixed_u64::convert(i128::from(a360))
    };
    match m361 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f39(v0: u64) -> u32 {
    let m365 = {
        let a364 = {
            let a362 = v0;
            let a363 = 4294967295u64;
            fixed_u64::bit_and(a362, a363)
        };
        fixed_u32::convert(i128::from(a364))
    };
    match m365 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f40(v0: u32, v1: u32) -> u32 {
    let m368 = {
        let a366 = v0;
        let a367 = v1;
        fixed_u32::shift_right(a366, a367)
    };
    match m368 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn run(p0: Bytes) -> R<Adt0> {
    f0(p0)
}
