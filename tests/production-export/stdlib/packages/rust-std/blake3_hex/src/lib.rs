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
    C0(u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Fn0 {
    F9(u64, Bytes, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
        match self {
            Fn0::F9(k0, k1, k2) => f9(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12(u64, u64, u64, Bytes, bool, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F12(k0, k1, k2, k3, k4, k5, k6) => f12(*k0, *k1, *k2, k3.clone(), *k4, *k5, *k6, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16(bool),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0), p1: Adt0) -> (u64, Adt0) {
        match self {
            Fn2::F16(k0) => f16(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
        match self {
            Fn3::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F24,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F24 => f24(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F26,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt1, Adt1)) -> (Adt1, Adt1) {
        match self {
            Fn5::F26 => f26(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    Ok(f1(f2(v0)?))
}

pub fn f1(v0: Adt0) -> Bytes {
    let a21 = f3(match v0.clone() {
        Adt0::C0(h1, _, _, _, _, _, _, _) => {
            h1
        }
    });
    let a22 = {
        let a19 = f3(match v0.clone() {
            Adt0::C0(_, h2, _, _, _, _, _, _) => {
                h2
            }
        });
        let a20 = {
            let a17 = f3(match v0.clone() {
                Adt0::C0(_, _, h3, _, _, _, _, _) => {
                    h3
                }
            });
            let a18 = {
                let a15 = f3(match v0.clone() {
                    Adt0::C0(_, _, _, h4, _, _, _, _) => {
                        h4
                    }
                });
                let a16 = {
                    let a13 = f3(match v0.clone() {
                        Adt0::C0(_, _, _, _, h5, _, _, _) => {
                            h5
                        }
                    });
                    let a14 = {
                        let a11 = f3(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h6, _, _) => {
                                h6
                            }
                        });
                        let a12 = {
                            let a9 = f3(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h7, _) => {
                                    h7
                                }
                            });
                            let a10 = f3(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h8) => {
                                    h8
                                }
                            });
                            append_bytes(a9, a10)
                        };
                        append_bytes(a11, a12)
                    };
                    append_bytes(a13, a14)
                };
                append_bytes(a15, a16)
            };
            append_bytes(a17, a18)
        };
        append_bytes(a19, a20)
    };
    append_bytes(a21, a22)
}

pub fn f2(v0: Bytes) -> R<Adt0> {
    f4(v0, 0u64, true)
}

pub fn f3(v0: u32) -> Bytes {
    let a51 = f5({
        let a23 = f6(v0, 4u32);
        let a24 = 15u32;
        fixed_u32::bit_and(a23, a24)
    });
    let a52 = {
        let a49 = f5({
            let a25 = v0;
            let a26 = 15u32;
            fixed_u32::bit_and(a25, a26)
        });
        let a50 = {
            let a47 = f5({
                let a27 = f6(v0, 12u32);
                let a28 = 15u32;
                fixed_u32::bit_and(a27, a28)
            });
            let a48 = {
                let a45 = f5({
                    let a29 = f6(v0, 8u32);
                    let a30 = 15u32;
                    fixed_u32::bit_and(a29, a30)
                });
                let a46 = {
                    let a43 = f5({
                        let a31 = f6(v0, 20u32);
                        let a32 = 15u32;
                        fixed_u32::bit_and(a31, a32)
                    });
                    let a44 = {
                        let a41 = f5({
                            let a33 = f6(v0, 16u32);
                            let a34 = 15u32;
                            fixed_u32::bit_and(a33, a34)
                        });
                        let a42 = {
                            let a39 = f5({
                                let a35 = f6(v0, 28u32);
                                let a36 = 15u32;
                                fixed_u32::bit_and(a35, a36)
                            });
                            let a40 = f5({
                                let a37 = f6(v0, 24u32);
                                let a38 = 15u32;
                                fixed_u32::bit_and(a37, a38)
                            });
                            append_bytes(a39, a40)
                        };
                        append_bytes(a41, a42)
                    };
                    append_bytes(a43, a44)
                };
                append_bytes(a45, a46)
            };
            append_bytes(a47, a48)
        };
        append_bytes(a49, a50)
    };
    append_bytes(a51, a52)
}

pub fn f4(v0: Bytes, v1: u64, v2: bool) -> R<Adt0> {
    let v3: u64 = {
        let a53 = v0.clone();
        length_bytes(a53)
    };
    let v4: u64 = {
        let a54 = v3;
        let a55 = 0u64;
        if nat_eq(a54, a55) {
            1u64
        } else {
            let a58 = {
                let a56 = v3;
                let a57 = 1023u64;
                nat_add(a56, a57)?
            };
            let a59 = 1024u64;
            let a60 = 0u64;
            nat_quot(a58, a59, a60)
        }
    };
    let v5: u64 = {
        let a61 = v4;
        let a62 = 1u64;
        nat_sub(a61, a62)
    };
    let a63 = v4;
    let a64 = 1u64;
    if nat_eq(a63, a64) {
        f7(v0, v3, 0u64, v1, v2)
    } else {
        let v6: List<Adt0> = {
            let (_, h65) = f8(Fn0::F9(v1, v0.clone(), v3), v5, (0u64, List::<Adt0>::nil()))?;
            h65
        };
        Ok(f10(v6, f7(v0, v3, v5, {
            let a66 = v1;
            let a67 = v5;
            nat_add(a66, a67)?
        }, false)?, v2))
    }
}

pub fn f5(v0: u32) -> Bytes {
    let a70 = {
        let a68 = v0;
        let a69 = 8u32;
        fixed_u32::bit_and(a68, a69)
    };
    let a71 = 0u32;
    if equal(a70, a71) {
        let a74 = {
            let a72 = v0;
            let a73 = 4u32;
            fixed_u32::bit_and(a72, a73)
        };
        let a75 = 0u32;
        if equal(a74, a75) {
            let a78 = {
                let a76 = v0;
                let a77 = 2u32;
                fixed_u32::bit_and(a76, a77)
            };
            let a79 = 0u32;
            if equal(a78, a79) {
                let a82 = {
                    let a80 = v0;
                    let a81 = 1u32;
                    fixed_u32::bit_and(a80, a81)
                };
                let a83 = 0u32;
                if equal(a82, a83) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a86 = {
                    let a84 = v0;
                    let a85 = 1u32;
                    fixed_u32::bit_and(a84, a85)
                };
                let a87 = 0u32;
                if equal(a86, a87) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a90 = {
                let a88 = v0;
                let a89 = 2u32;
                fixed_u32::bit_and(a88, a89)
            };
            let a91 = 0u32;
            if equal(a90, a91) {
                let a94 = {
                    let a92 = v0;
                    let a93 = 1u32;
                    fixed_u32::bit_and(a92, a93)
                };
                let a95 = 0u32;
                if equal(a94, a95) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a98 = {
                    let a96 = v0;
                    let a97 = 1u32;
                    fixed_u32::bit_and(a96, a97)
                };
                let a99 = 0u32;
                if equal(a98, a99) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a102 = {
            let a100 = v0;
            let a101 = 4u32;
            fixed_u32::bit_and(a100, a101)
        };
        let a103 = 0u32;
        if equal(a102, a103) {
            let a106 = {
                let a104 = v0;
                let a105 = 2u32;
                fixed_u32::bit_and(a104, a105)
            };
            let a107 = 0u32;
            if equal(a106, a107) {
                let a110 = {
                    let a108 = v0;
                    let a109 = 1u32;
                    fixed_u32::bit_and(a108, a109)
                };
                let a111 = 0u32;
                if equal(a110, a111) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a114 = {
                    let a112 = v0;
                    let a113 = 1u32;
                    fixed_u32::bit_and(a112, a113)
                };
                let a115 = 0u32;
                if equal(a114, a115) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a118 = {
                let a116 = v0;
                let a117 = 2u32;
                fixed_u32::bit_and(a116, a117)
            };
            let a119 = 0u32;
            if equal(a118, a119) {
                let a122 = {
                    let a120 = v0;
                    let a121 = 1u32;
                    fixed_u32::bit_and(a120, a121)
                };
                let a123 = 0u32;
                if equal(a122, a123) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a126 = {
                    let a124 = v0;
                    let a125 = 1u32;
                    fixed_u32::bit_and(a124, a125)
                };
                let a127 = 0u32;
                if equal(a126, a127) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f6(v0: u32, v1: u32) -> u32 {
    let m130 = {
        let a128 = v0;
        let a129 = v1;
        fixed_u32::shift_right(a128, a129)
    };
    match m130 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = {
        let a131 = v2;
        let a132 = 1024u64;
        nat_mul(a131, a132)?
    };
    let v6: u64 = {
        let a133 = v1;
        let a134 = v5;
        nat_sub(a133, a134)
    };
    let v7: u64 = {
        let a135 = v6;
        let a136 = 1024u64;
        if nat_lt(a135, a136) {
            v6
        } else {
            1024u64
        }
    };
    let v8: u64 = {
        let a137 = v7;
        let a138 = 0u64;
        if nat_eq(a137, a138) {
            1u64
        } else {
            let a141 = {
                let a139 = v7;
                let a140 = 63u64;
                nat_add(a139, a140)?
            };
            let a142 = 64u64;
            let a143 = 0u64;
            nat_quot(a141, a142, a143)
        }
    };
    let (_, h144) = f11(Fn1::F12(v8, v3, v7, v0, v4, v1, v5), v8, (0u64, f13()))?;
    Ok(h144)
}

pub fn f8(v0: Fn0, p148: u64, p149: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v1, v2) = m147;
                let m145 = v1;
                if m145 == 0 {
                    Err::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>(Ok(v2))
                } else {
                    let v3 = m145 - 1;
                    Ok::<(u64, (u64, List<Adt0>)), R<(u64, List<Adt0>)>>((v3, {
                        let c146: Fn0 = v0.clone();
                        c146.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64, v1: Bytes, v2: u64, v3: (u64, List<Adt0>)) -> R<(u64, List<Adt0>)> {
    let v4: u64 = {
        let (h150, _) = v3;
        h150
    };
    Ok(({
        let a151 = v4;
        let a152 = 1u64;
        nat_add(a151, a152)?
    }, f14({
        let (_, h153) = v3;
        h153
    }, f7(v1, v2, v4, {
        let a154 = v0;
        let a155 = v4;
        nat_add(a154, a155)?
    }, false)?, {
        let a156 = v4;
        let a157 = 1u64;
        nat_add(a156, a157)?
    })))
}

pub fn f10(v0: List<Adt0>, v1: Adt0, v2: bool) -> Adt0 {
    let (_, h158) = f15(Fn2::F16(v2), ({
        let a159 = v0.clone();
        length_list(a159)
    }, v1), v0);
    h158
}

pub fn f11(v0: Fn1, p163: u64, p164: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m160 = v1;
                if m160 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m160 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c161: Fn1 = v0.clone();
                        c161.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: Bytes, v4: bool, v5: u64, v6: u64, v7: (u64, Adt0)) -> R<(u64, Adt0)> {
    let v8: u64 = {
        let (h165, _) = v7;
        h165
    };
    let v9: bool = {
        let a168 = {
            let a166 = v8;
            let a167 = 1u64;
            nat_add(a166, a167)?
        };
        let a169 = v0;
        nat_eq(a168, a169)
    };
    let v10: u32 = {
        let a172 = {
            let a170 = v8;
            let a171 = 0u64;
            if nat_eq(a170, a171) {
                1u32
            } else {
                0u32
            }
        };
        let a173 = if v9 {
            if v4 {
                10u32
            } else {
                2u32
            }
        } else {
            0u32
        };
        fixed_u32::bit_or(a172, a173)
    };
    let v11: u32 = if v9 {
        f17({
            let a176 = v2;
            let a177 = {
                let a174 = v8;
                let a175 = 64u64;
                nat_mul(a174, a175)?
            };
            nat_sub(a176, a177)
        })
    } else {
        64u32
    };
    Ok(({
        let a178 = v8;
        let a179 = 1u64;
        nat_add(a178, a179)?
    }, f18({
        let (_, h180) = v7;
        h180
    }, f19(v3, v5, {
        let a183 = v6;
        let a184 = {
            let a181 = v8;
            let a182 = 64u64;
            nat_mul(a181, a182)?
        };
        nat_add(a183, a184)?
    })?, v1, v11, v10)))
}

pub fn f13() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f14(v0: List<Adt0>, v1: Adt0, v2: u64) -> List<Adt0> {
    let v3: (u64, (Adt0, List<Adt0>)) = f20(Fn3::F21, 64u64, (v2, (v1, v0)));
    List::cons({
        let (h185, _) = {
            let (_, h186) = v3.clone();
            h186
        };
        h185
    }, {
        let (_, h187) = {
            let (_, h188) = v3;
            h188
        };
        h187
    })
}

pub fn f15(v0: Fn2, p192: (u64, Adt0), p193: List<Adt0>) -> (u64, Adt0) {
    let m191 = (p192, p193);
    {
        let mut m191 = m191;
        loop {
            let step = {
                let (v1, v2) = m191;
                let m189 = v2;
                match m189.uncons() {
                    None => {
                        Err::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Adt0), List<Adt0>), (u64, Adt0)>(({
                            let c190: Fn2 = v0.clone();
                            c190.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: bool, v1: (u64, Adt0), v2: Adt0) -> (u64, Adt0) {
    ({
        let a195 = {
            let (h194, _) = v1.clone();
            h194
        };
        let a196 = 1u64;
        nat_sub(a195, a196)
    }, f22(v2, {
        let (_, h197) = v1;
        h197
    }, if v0 {
        let a199 = {
            let (h198, _) = v1;
            h198
        };
        let a200 = 1u64;
        nat_eq(a199, a200)
    } else {
        false
    }))
}

pub fn f17(v0: u64) -> u32 {
    let (_, h201) = {
        let (_, h202) = f23(Fn4::F24, 32u64, (v0, (1u32, 0u32)));
        h202
    };
    h201
}

pub fn f18(v0: Adt0, v1: Adt1, v2: u64, v3: u32, v4: u32) -> Adt0 {
    let v5: Adt1 = Adt1::C0(match v0.clone() {
        Adt0::C0(h203, _, _, _, _, _, _, _) => {
            h203
        }
    }, match v0.clone() {
        Adt0::C0(_, h204, _, _, _, _, _, _) => {
            h204
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h205, _, _, _, _, _) => {
            h205
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h206, _, _, _, _) => {
            h206
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h207, _, _, _) => {
            h207
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h208, _, _) => {
            h208
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h209, _) => {
            h209
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h210) => {
            h210
        }
    }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f17({
        let a211 = v2;
        let a212 = 4294967296u64;
        let a213 = 0u64;
        nat_rem(a211, a212, a213)
    }), f17({
        let a214 = v2;
        let a215 = 4294967296u64;
        let a216 = 0u64;
        nat_quot(a214, a215, a216)
    }), v3, v4);
    let v6: Adt1 = {
        let (h217, _) = f25(Fn5::F26, 7u64, (v5, v1));
        h217
    };
    Adt0::C0({
        let a220 = match v6.clone() {
            Adt1::C0(h218, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h218
            }
        };
        let a221 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h219, _, _, _, _, _, _, _) => {
                h219
            }
        };
        fixed_u32::bit_xor(a220, a221)
    }, {
        let a224 = match v6.clone() {
            Adt1::C0(_, h222, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        };
        let a225 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h223, _, _, _, _, _, _) => {
                h223
            }
        };
        fixed_u32::bit_xor(a224, a225)
    }, {
        let a228 = match v6.clone() {
            Adt1::C0(_, _, h226, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a229 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h227, _, _, _, _, _) => {
                h227
            }
        };
        fixed_u32::bit_xor(a228, a229)
    }, {
        let a232 = match v6.clone() {
            Adt1::C0(_, _, _, h230, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h231, _, _, _, _) => {
                h231
            }
        };
        fixed_u32::bit_xor(a232, a233)
    }, {
        let a236 = match v6.clone() {
            Adt1::C0(_, _, _, _, h234, _, _, _, _, _, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h235, _, _, _) => {
                h235
            }
        };
        fixed_u32::bit_xor(a236, a237)
    }, {
        let a240 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, h238, _, _, _, _, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h239, _, _) => {
                h239
            }
        };
        fixed_u32::bit_xor(a240, a241)
    }, {
        let a244 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, h242, _, _, _, _, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h243, _) => {
                h243
            }
        };
        fixed_u32::bit_xor(a244, a245)
    }, {
        let a248 = match v6.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h246, _, _, _, _, _, _, _, _) => {
                h246
            }
        };
        let a249 = match v6 {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h247) => {
                h247
            }
        };
        fixed_u32::bit_xor(a248, a249)
    })
}

pub fn f19(v0: Bytes, v1: u64, v2: u64) -> R<Adt1> {
    Ok(Adt1::C0(f27(v0.clone(), v1, v2)?, f27(v0.clone(), v1, {
        let a250 = v2;
        let a251 = 4u64;
        nat_add(a250, a251)?
    })?, f27(v0.clone(), v1, {
        let a252 = v2;
        let a253 = 8u64;
        nat_add(a252, a253)?
    })?, f27(v0.clone(), v1, {
        let a254 = v2;
        let a255 = 12u64;
        nat_add(a254, a255)?
    })?, f27(v0.clone(), v1, {
        let a256 = v2;
        let a257 = 16u64;
        nat_add(a256, a257)?
    })?, f27(v0.clone(), v1, {
        let a258 = v2;
        let a259 = 20u64;
        nat_add(a258, a259)?
    })?, f27(v0.clone(), v1, {
        let a260 = v2;
        let a261 = 24u64;
        nat_add(a260, a261)?
    })?, f27(v0.clone(), v1, {
        let a262 = v2;
        let a263 = 28u64;
        nat_add(a262, a263)?
    })?, f27(v0.clone(), v1, {
        let a264 = v2;
        let a265 = 32u64;
        nat_add(a264, a265)?
    })?, f27(v0.clone(), v1, {
        let a266 = v2;
        let a267 = 36u64;
        nat_add(a266, a267)?
    })?, f27(v0.clone(), v1, {
        let a268 = v2;
        let a269 = 40u64;
        nat_add(a268, a269)?
    })?, f27(v0.clone(), v1, {
        let a270 = v2;
        let a271 = 44u64;
        nat_add(a270, a271)?
    })?, f27(v0.clone(), v1, {
        let a272 = v2;
        let a273 = 48u64;
        nat_add(a272, a273)?
    })?, f27(v0.clone(), v1, {
        let a274 = v2;
        let a275 = 52u64;
        nat_add(a274, a275)?
    })?, f27(v0.clone(), v1, {
        let a276 = v2;
        let a277 = 56u64;
        nat_add(a276, a277)?
    })?, f27(v0, v1, {
        let a278 = v2;
        let a279 = 60u64;
        nat_add(a278, a279)?
    })?))
}

pub fn f20(v0: Fn3, p283: u64, p284: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v1;
                if m280 == 0 {
                    Err::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>(v2)
                } else {
                    let v3 = m280 - 1;
                    Ok::<(u64, (u64, (Adt0, List<Adt0>))), (u64, (Adt0, List<Adt0>))>((v3, {
                        let c281: Fn3 = v0.clone();
                        c281.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: (u64, (Adt0, List<Adt0>))) -> (u64, (Adt0, List<Adt0>)) {
    let v1: u64 = {
        let (h285, _) = v0.clone();
        h285
    };
    let v2: Adt0 = {
        let (h286, _) = {
            let (_, h287) = v0.clone();
            h287
        };
        h286
    };
    let v3: List<Adt0> = {
        let (_, h288) = {
            let (_, h289) = v0.clone();
            h289
        };
        h288
    };
    let a293 = {
        let a290 = v1;
        let a291 = 2u64;
        let a292 = 0u64;
        nat_rem(a290, a291, a292)
    };
    let a294 = 0u64;
    if nat_eq(a293, a294) {
        let m295 = v3;
        match m295.uncons() {
            None => {
                v0
            }
            Some((v4, v5)) => {
                ({
                    let a296 = v1;
                    let a297 = 2u64;
                    let a298 = 0u64;
                    nat_quot(a296, a297, a298)
                }, (f22(v4, v2, false), v5))
            }
        }
    } else {
        v0
    }
}

pub fn f22(v0: Adt0, v1: Adt0, v2: bool) -> Adt0 {
    f18(f13(), Adt1::C0(match v0.clone() {
        Adt0::C0(h299, _, _, _, _, _, _, _) => {
            h299
        }
    }, match v0.clone() {
        Adt0::C0(_, h300, _, _, _, _, _, _) => {
            h300
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h301, _, _, _, _, _) => {
            h301
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h302, _, _, _, _) => {
            h302
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h303, _, _, _) => {
            h303
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h304, _, _) => {
            h304
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h305, _) => {
            h305
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h306) => {
            h306
        }
    }, match v1.clone() {
        Adt0::C0(h307, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v1.clone() {
        Adt0::C0(_, h308, _, _, _, _, _, _) => {
            h308
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h309, _, _, _, _, _) => {
            h309
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h310, _, _, _, _) => {
            h310
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h311, _, _, _) => {
            h311
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h312, _, _) => {
            h312
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, h313, _) => {
            h313
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h314) => {
            h314
        }
    }), 0u64, 64u32, if v2 {
        12u32
    } else {
        4u32
    })
}

pub fn f23(v0: Fn4, p318: u64, p319: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m317 = (p318, p319);
    {
        let mut m317 = m317;
        loop {
            let step = {
                let (v1, v2) = m317;
                let m315 = v1;
                if m315 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m315 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c316: Fn4 = v0.clone();
                        c316.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h320, _) = v0;
        h320
    };
    let v2: u32 = {
        let (h321, _) = {
            let (_, h322) = v0;
            h322
        };
        h321
    };
    let v3: u32 = {
        let (_, h323) = {
            let (_, h324) = v0;
            h324
        };
        h323
    };
    ({
        let a325 = v1;
        let a326 = 2u64;
        let a327 = 0u64;
        nat_quot(a325, a326, a327)
    }, (f28(v2, 1u32), {
        let a331 = {
            let a328 = v1;
            let a329 = 2u64;
            let a330 = 0u64;
            nat_rem(a328, a329, a330)
        };
        let a332 = 1u64;
        if nat_eq(a331, a332) {
            let a333 = v3;
            let a334 = v2;
            fixed_u32::bit_or(a333, a334)
        } else {
            v3
        }
    }))
}

pub fn f25(v0: Fn5, p338: u64, p339: (Adt1, Adt1)) -> (Adt1, Adt1) {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v1;
                if m335 == 0 {
                    Err::<(u64, (Adt1, Adt1)), (Adt1, Adt1)>(v2)
                } else {
                    let v3 = m335 - 1;
                    Ok::<(u64, (Adt1, Adt1)), (Adt1, Adt1)>((v3, {
                        let c336: Fn5 = v0.clone();
                        c336.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (Adt1, Adt1)) -> (Adt1, Adt1) {
    (f29({
        let (h340, _) = v0.clone();
        h340
    }, {
        let (_, h341) = v0.clone();
        h341
    }), f30({
        let (_, h342) = v0;
        h342
    }))
}

pub fn f27(v0: Bytes, v1: u64, v2: u64) -> R<u32> {
    let a353 = f31(v0.clone(), v1, v2);
    let a354 = {
        let a351 = f28(f31(v0.clone(), v1, {
            let a343 = v2;
            let a344 = 1u64;
            nat_add(a343, a344)?
        }), 8u32);
        let a352 = {
            let a349 = f28(f31(v0.clone(), v1, {
                let a345 = v2;
                let a346 = 2u64;
                nat_add(a345, a346)?
            }), 16u32);
            let a350 = f28(f31(v0, v1, {
                let a347 = v2;
                let a348 = 3u64;
                nat_add(a347, a348)?
            }), 24u32);
            fixed_u32::bit_or(a349, a350)
        };
        fixed_u32::bit_or(a351, a352)
    };
    Ok(fixed_u32::bit_or(a353, a354))
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m357 = {
        let a355 = v0;
        let a356 = v1;
        fixed_u32::shift_left(a355, a356)
    };
    match m357 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> Adt1 {
    let v2: Adt2 = f32(match v0.clone() {
        Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h358
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h359, _, _, _, _, _, _, _, _, _, _, _) => {
            h359
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h360, _, _, _, _, _, _, _) => {
            h360
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h361, _, _, _) => {
            h361
        }
    }, match v1.clone() {
        Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h362
        }
    }, match v1.clone() {
        Adt1::C0(_, h363, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h363
        }
    });
    let v3: Adt2 = f32(match v0.clone() {
        Adt1::C0(_, h364, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h364
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h365, _, _, _, _, _, _, _, _, _, _) => {
            h365
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h366, _, _, _, _, _, _) => {
            h366
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h367, _, _) => {
            h367
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h368, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h368
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h369, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h369
        }
    });
    let v4: Adt2 = f32(match v0.clone() {
        Adt1::C0(_, _, h370, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h370
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h371, _, _, _, _, _, _, _, _, _) => {
            h371
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h372, _, _, _, _, _) => {
            h372
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h373, _) => {
            h373
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h374, _, _, _, _, _, _, _, _, _, _, _) => {
            h374
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h375, _, _, _, _, _, _, _, _, _, _) => {
            h375
        }
    });
    let v5: Adt2 = f32(match v0.clone() {
        Adt1::C0(_, _, _, h376, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h376
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h377, _, _, _, _, _, _, _, _) => {
            h377
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h378, _, _, _, _) => {
            h378
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h379) => {
            h379
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h380, _, _, _, _, _, _, _, _, _) => {
            h380
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h381, _, _, _, _, _, _, _, _) => {
            h381
        }
    });
    let v6: Adt2 = f32(match v2.clone() {
        Adt2::C0(h382, _, _, _) => {
            h382
        }
    }, match v3.clone() {
        Adt2::C0(_, h383, _, _) => {
            h383
        }
    }, match v4.clone() {
        Adt2::C0(_, _, h384, _) => {
            h384
        }
    }, match v5.clone() {
        Adt2::C0(_, _, _, h385) => {
            h385
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h386, _, _, _, _, _, _, _) => {
            h386
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h387, _, _, _, _, _, _) => {
            h387
        }
    });
    let v7: Adt2 = f32(match v3.clone() {
        Adt2::C0(h388, _, _, _) => {
            h388
        }
    }, match v4.clone() {
        Adt2::C0(_, h389, _, _) => {
            h389
        }
    }, match v5.clone() {
        Adt2::C0(_, _, h390, _) => {
            h390
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h391) => {
            h391
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h392, _, _, _, _, _) => {
            h392
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h393, _, _, _, _) => {
            h393
        }
    });
    let v8: Adt2 = f32(match v4.clone() {
        Adt2::C0(h394, _, _, _) => {
            h394
        }
    }, match v5.clone() {
        Adt2::C0(_, h395, _, _) => {
            h395
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h396, _) => {
            h396
        }
    }, match v3.clone() {
        Adt2::C0(_, _, _, h397) => {
            h397
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h398, _, _, _) => {
            h398
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h399, _, _) => {
            h399
        }
    });
    let v9: Adt2 = f32(match v5 {
        Adt2::C0(h400, _, _, _) => {
            h400
        }
    }, match v2 {
        Adt2::C0(_, h401, _, _) => {
            h401
        }
    }, match v3 {
        Adt2::C0(_, _, h402, _) => {
            h402
        }
    }, match v4 {
        Adt2::C0(_, _, _, h403) => {
            h403
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h404, _) => {
            h404
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h405) => {
            h405
        }
    });
    Adt1::C0(match v6.clone() {
        Adt2::C0(h406, _, _, _) => {
            h406
        }
    }, match v7.clone() {
        Adt2::C0(h407, _, _, _) => {
            h407
        }
    }, match v8.clone() {
        Adt2::C0(h408, _, _, _) => {
            h408
        }
    }, match v9.clone() {
        Adt2::C0(h409, _, _, _) => {
            h409
        }
    }, match v9.clone() {
        Adt2::C0(_, h410, _, _) => {
            h410
        }
    }, match v6.clone() {
        Adt2::C0(_, h411, _, _) => {
            h411
        }
    }, match v7.clone() {
        Adt2::C0(_, h412, _, _) => {
            h412
        }
    }, match v8.clone() {
        Adt2::C0(_, h413, _, _) => {
            h413
        }
    }, match v8.clone() {
        Adt2::C0(_, _, h414, _) => {
            h414
        }
    }, match v9.clone() {
        Adt2::C0(_, _, h415, _) => {
            h415
        }
    }, match v6.clone() {
        Adt2::C0(_, _, h416, _) => {
            h416
        }
    }, match v7.clone() {
        Adt2::C0(_, _, h417, _) => {
            h417
        }
    }, match v7 {
        Adt2::C0(_, _, _, h418) => {
            h418
        }
    }, match v8 {
        Adt2::C0(_, _, _, h419) => {
            h419
        }
    }, match v9 {
        Adt2::C0(_, _, _, h420) => {
            h420
        }
    }, match v6 {
        Adt2::C0(_, _, _, h421) => {
            h421
        }
    })
}

pub fn f30(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, _, h422, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h422
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h423, _, _, _, _, _, _, _, _, _) => {
            h423
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h424, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h425, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h426, _, _, _, _, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt1::C0(h427, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h428, _, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h429, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt1::C0(_, h430, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h431, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h432, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h433, _, _, _, _, _, _, _, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h434, _, _, _, _, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h435, _) => {
            h435
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h436) => {
            h436
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h437, _, _, _, _, _, _, _) => {
            h437
        }
    })
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a438 = v2;
    let a439 = v1;
    if nat_lt(a438, a439) {
        let m442 = {
            let a440 = v0;
            let a441 = v2;
            index_bytes(a440, a441)
        };
        match m442 {
            None => {
                0u32
            }
            Some(v3) => {
                let m444 = {
                    let a443 = v3;
                    fixed_u32::convert(i128::from(a443))
                };
                match m444 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        0u32
    }
}

pub fn f32(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt2 {
    let v6: u32 = f33(f33(v0, v1), v4);
    let v7: u32 = f34({
        let a445 = v3;
        let a446 = v6;
        fixed_u32::bit_xor(a445, a446)
    }, 16u32, 16u32);
    let v8: u32 = f33(v2, v7);
    let v9: u32 = f34({
        let a447 = v1;
        let a448 = v8;
        fixed_u32::bit_xor(a447, a448)
    }, 12u32, 20u32);
    let v10: u32 = f33(f33(v6, v9), v5);
    let v11: u32 = f34({
        let a449 = v7;
        let a450 = v10;
        fixed_u32::bit_xor(a449, a450)
    }, 8u32, 24u32);
    let v12: u32 = f33(v8, v11);
    let v13: u32 = f34({
        let a451 = v9;
        let a452 = v12;
        fixed_u32::bit_xor(a451, a452)
    }, 7u32, 25u32);
    Adt2::C0(v10, v13, v12, v11)
}

pub fn f33(v0: u32, v1: u32) -> u32 {
    let m455 = {
        let a453 = f35(v0);
        let a454 = f35(v1);
        fixed_u64::checked_add(a453, a454)
    };
    match m455 {
        None => {
            0u32
        }
        Some(v2) => {
            f36(v2)
        }
    }
}

pub fn f34(v0: u32, v1: u32, v2: u32) -> u32 {
    let a456 = f6(v0, v1);
    let a457 = f28(v0, v2);
    fixed_u32::bit_or(a456, a457)
}

pub fn f35(v0: u32) -> u64 {
    let m459 = {
        let a458 = v0;
        fixed_u64::convert(i128::from(a458))
    };
    match m459 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f36(v0: u64) -> u32 {
    let m463 = {
        let a462 = {
            let a460 = v0;
            let a461 = 4294967295u64;
            fixed_u64::bit_and(a460, a461)
        };
        fixed_u32::convert(i128::from(a462))
    };
    match m463 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn run(p0: Bytes) -> R<Bytes> {
    f0(p0)
}
