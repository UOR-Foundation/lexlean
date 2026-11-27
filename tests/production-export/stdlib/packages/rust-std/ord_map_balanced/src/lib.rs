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
    C0,
    C1(std::rc::Rc<Adt0>, Bytes, Bytes, u64, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F7,
}

impl Fn0 {
    pub fn apply(&self, p0: (List<Option<Bytes>>, Adt0), p1: (u64, (Bytes, Bytes))) -> R<(List<Option<Bytes>>, Adt0)> {
        match self {
            Fn0::F7 => f7(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, Option<Bytes>), p1: (Bytes, Bytes)) -> (bool, Option<Bytes>) {
        match self {
            Fn1::F11 => f11(p0, p1),
        }
    }
}

pub fn f0(v0: List<(u64, (Bytes, Bytes))>) -> R<bool> {
    f1({
        let (_, h1) = f2(v0)?;
        h1
    })
}

pub fn f1(v0: Adt0) -> R<bool> {
    let (h2, _) = f3(v0.clone())?;
    if h2 {
        Ok(f4(f5(v0)))
    } else {
        Ok(false)
    }
}

pub fn f2(v0: List<(u64, (Bytes, Bytes))>) -> R<(List<Option<Bytes>>, Adt0)> {
    f6(Fn0::F7, (List::<Option<Bytes>>::nil(), f8()), v0)
}

pub fn f3(v0: Adt0) -> R<(bool, u64)> {
    let m3 = v0;
    match m3 {
        Adt0::C0 => {
            Ok((true, 0u64))
        }
        Adt0::C1(r24, _, _, v4, r25) => {
            let v1 = (*r24).clone();
            let v5 = (*r25).clone();
            let v6: (bool, u64) = f3(v1)?;
            let v7: (bool, u64) = f3(v5)?;
            let v8: u64 = {
                let a6 = f9({
                    let (_, h4) = v6;
                    h4
                }, {
                    let (_, h5) = v7;
                    h5
                });
                let a7 = 1u64;
                nat_add(a6, a7)?
            };
            Ok(({
                let (h8, _) = v6;
                if h8 {
                    let (h9, _) = v7;
                    if h9 {
                        let a10 = v4;
                        let a11 = v8;
                        if equal(a10, a11) {
                            let a16 = {
                                let (_, h12) = v6;
                                h12
                            };
                            let a17 = {
                                let a14 = {
                                    let (_, h13) = v7;
                                    h13
                                };
                                let a15 = 2u64;
                                nat_add(a14, a15)?
                            };
                            if nat_lt(a16, a17) {
                                let a22 = {
                                    let (_, h18) = v7;
                                    h18
                                };
                                let a23 = {
                                    let a20 = {
                                        let (_, h19) = v6;
                                        h19
                                    };
                                    let a21 = 2u64;
                                    nat_add(a20, a21)?
                                };
                                nat_lt(a22, a23)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }, v8))
        }
    }
}

pub fn f4(v0: List<(Bytes, Bytes)>) -> bool {
    let (h26, _) = f10(Fn1::F11, (true, None::<Bytes>), v0);
    h26
}

pub fn f5(v0: Adt0) -> List<(Bytes, Bytes)> {
    f12(v0, List::<(Bytes, Bytes)>::nil())
}

pub fn f6(v0: Fn0, p30: (List<Option<Bytes>>, Adt0), p31: List<(u64, (Bytes, Bytes))>) -> R<(List<Option<Bytes>>, Adt0)> {
    let m29 = (p30, p31);
    {
        let mut m29 = m29;
        loop {
            let step = {
                let (v1, v2) = m29;
                let m27 = v2;
                match m27.uncons() {
                    None => {
                        Err::<((List<Option<Bytes>>, Adt0), List<(u64, (Bytes, Bytes))>), R<(List<Option<Bytes>>, Adt0)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Option<Bytes>>, Adt0), List<(u64, (Bytes, Bytes))>), R<(List<Option<Bytes>>, Adt0)>>(({
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

pub fn f7(v0: (List<Option<Bytes>>, Adt0), v1: (u64, (Bytes, Bytes))) -> R<(List<Option<Bytes>>, Adt0)> {
    f13(v0, v1)
}

pub fn f8() -> Adt0 {
    Adt0::C0
}

pub fn f9(v0: u64, v1: u64) -> u64 {
    let a32 = v0;
    let a33 = v1;
    if nat_lt(a32, a33) {
        v1
    } else {
        v0
    }
}

pub fn f10(v0: Fn1, p37: (bool, Option<Bytes>), p38: List<(Bytes, Bytes)>) -> (bool, Option<Bytes>) {
    let m36 = (p37, p38);
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, v2) = m36;
                let m34 = v2;
                match m34.uncons() {
                    None => {
                        Err::<((bool, Option<Bytes>), List<(Bytes, Bytes)>), (bool, Option<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Option<Bytes>), List<(Bytes, Bytes)>), (bool, Option<Bytes>)>(({
                            let c35: Fn1 = v0.clone();
                            c35.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: (bool, Option<Bytes>), v1: (Bytes, Bytes)) -> (bool, Option<Bytes>) {
    let m40 = {
        let (_, h39) = v0;
        h39
    };
    match m40 {
        None => {
            ({
                let (h41, _) = v0;
                h41
            }, Some({
                let (h42, _) = v1;
                h42
            }))
        }
        Some(v2) => {
            ({
                let (h43, _) = v0;
                if h43 {
                    f14(v2, {
                        let (h44, _) = v1.clone();
                        h44
                    })
                } else {
                    false
                }
            }, Some({
                let (h45, _) = v1;
                h45
            }))
        }
    }
}

pub fn f12(v0: Adt0, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m46 = v0;
    match m46 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1(r47, v3, v4, _, r48) => {
            let v2 = (*r47).clone();
            let v6 = (*r48).clone();
            f12(v2, List::cons((v3, v4), f12(v6, v1)))
        }
    }
}

pub fn f13(v0: (List<Option<Bytes>>, Adt0), v1: (u64, (Bytes, Bytes))) -> R<(List<Option<Bytes>>, Adt0)> {
    let v2: Adt0 = {
        let (_, h49) = v0;
        h49
    };
    let v3: Bytes = {
        let (h50, _) = {
            let (_, h51) = v1.clone();
            h51
        };
        h50
    };
    let v4: Option<Bytes> = f15(v2.clone(), v3.clone());
    let v5: List<Option<Bytes>> = List::cons(v4, {
        let (h52, _) = v0;
        h52
    });
    let a54 = {
        let (h53, _) = v1.clone();
        h53
    };
    let a55 = 0u64;
    if nat_eq(a54, a55) {
        Ok((v5, f16(v2, v3, {
            let (_, h56) = {
                let (_, h57) = v1;
                h57
            };
            h56
        })?))
    } else {
        let a59 = {
            let (h58, _) = v1;
            h58
        };
        let a60 = 1u64;
        if nat_eq(a59, a60) {
            Ok((v5, f17(v2, v3)?))
        } else {
            Ok((v5, v2))
        }
    }
}

pub fn f14(v0: Bytes, v1: Bytes) -> bool {
    let a65 = {
        let a61 = v0;
        let a62 = v1;
        compare_bytes(a61, a62)
    };
    let a66 = {
        let a63 = Bytes::lit(&[]);
        let a64 = Bytes::lit(&[0x01]);
        compare_bytes(a63, a64)
    };
    equal(a65, a66)
}

pub fn f15(p73: Adt0, v1: Bytes) -> Option<Bytes> {
    let m72 = p73;
    {
        let mut m72 = m72;
        loop {
            let step = {
                let v0 = m72;
                let m67 = v0;
                match m67 {
                    Adt0::C0 => {
                        Err::<Adt0, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt0::C1(r70, v3, v4, _, r71) => {
                        let v2 = (*r70).clone();
                        let v6 = (*r71).clone();
                        let a68 = v1.clone();
                        let a69 = v3.clone();
                        if equal(a68, a69) {
                            Err::<Adt0, Option<Bytes>>(Some(v4))
                        } else {
                            if f14(v1.clone(), v3) {
                                Ok::<Adt0, Option<Bytes>>(v2)
                            } else {
                                Ok::<Adt0, Option<Bytes>>(v6)
                            }
                        }
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

pub fn f16(v0: Adt0, v1: Bytes, v2: Bytes) -> R<Adt0> {
    let m74 = v0;
    match m74 {
        Adt0::C0 => {
            Ok(Adt0::C1(std::rc::Rc::new(Adt0::C0), v1, v2, 1u64, std::rc::Rc::new(Adt0::C0)))
        }
        Adt0::C1(r77, v4, v5, v6, r78) => {
            let v3 = (*r77).clone();
            let v7 = (*r78).clone();
            let a75 = v1.clone();
            let a76 = v4.clone();
            if equal(a75, a76) {
                Ok(Adt0::C1(std::rc::Rc::new(v3), v1, v2, v6, std::rc::Rc::new(v7)))
            } else {
                if f14(v1.clone(), v4.clone()) {
                    f18(f16(v3, v1, v2)?, v4, v5, v7)
                } else {
                    f18(v3, v4, v5, f16(v7, v1, v2)?)
                }
            }
        }
    }
}

pub fn f17(v0: Adt0, v1: Bytes) -> R<Adt0> {
    let m79 = v0;
    match m79 {
        Adt0::C0 => {
            Ok(Adt0::C0)
        }
        Adt0::C1(r82, v3, v4, _, r83) => {
            let v2 = (*r82).clone();
            let v6 = (*r83).clone();
            let a80 = v1.clone();
            let a81 = v3.clone();
            if equal(a80, a81) {
                f19(v2, v6)
            } else {
                if f14(v1.clone(), v3.clone()) {
                    f18(f17(v2, v1)?, v3, v4, v6)
                } else {
                    f18(v2, v3, v4, f17(v6, v1)?)
                }
            }
        }
    }
}

pub fn f18(v0: Adt0, v1: Bytes, v2: Bytes, v3: Adt0) -> R<Adt0> {
    let v4: u64 = f20(v0.clone());
    let v5: u64 = f20(v3.clone());
    let a86 = {
        let a84 = v5;
        let a85 = 1u64;
        nat_add(a84, a85)?
    };
    let a87 = v4;
    if nat_lt(a86, a87) {
        f21(v0, v1, v2, v3)
    } else {
        let a90 = {
            let a88 = v4;
            let a89 = 1u64;
            nat_add(a88, a89)?
        };
        let a91 = v5;
        if nat_lt(a90, a91) {
            f22(v0, v1, v2, v3)
        } else {
            f23(v0, v1, v2, v3)
        }
    }
}

pub fn f19(v0: Adt0, v1: Adt0) -> R<Adt0> {
    let v2: (Option<(Bytes, Bytes)>, Adt0) = f24(v1)?;
    let m93 = {
        let (h92, _) = v2;
        h92
    };
    match m93 {
        None => {
            Ok(v0)
        }
        Some(v3) => {
            f18(v0, {
                let (h94, _) = v3;
                h94
            }, {
                let (_, h95) = v3;
                h95
            }, {
                let (_, h96) = v2;
                h96
            })
        }
    }
}

pub fn f20(v0: Adt0) -> u64 {
    let m97 = v0;
    match m97 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f21(v0: Adt0, v1: Bytes, v2: Bytes, v3: Adt0) -> R<Adt0> {
    let m98 = v0.clone();
    match m98 {
        Adt0::C0 => {
            f23(v0, v1, v2, v3)
        }
        Adt0::C1(r106, v5, v6, _, r107) => {
            let v4 = (*r106).clone();
            let v8 = (*r107).clone();
            let a101 = f20(v8.clone());
            let a102 = {
                let a99 = f20(v4.clone());
                let a100 = 1u64;
                nat_add(a99, a100)?
            };
            if nat_lt(a101, a102) {
                f23(v4, v5, v6, f23(v8, v1, v2, v3)?)
            } else {
                let m103 = v8;
                match m103 {
                    Adt0::C0 => {
                        f23(v0, v1, v2, v3)
                    }
                    Adt0::C1(r104, v10, v11, _, r105) => {
                        let v9 = (*r104).clone();
                        let v13 = (*r105).clone();
                        f23(f23(v4, v5, v6, v9)?, v10, v11, f23(v13, v1, v2, v3)?)
                    }
                }
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Bytes, v2: Bytes, v3: Adt0) -> R<Adt0> {
    let m108 = v3.clone();
    match m108 {
        Adt0::C0 => {
            f23(v0, v1, v2, v3)
        }
        Adt0::C1(r116, v5, v6, _, r117) => {
            let v4 = (*r116).clone();
            let v8 = (*r117).clone();
            let a111 = f20(v4.clone());
            let a112 = {
                let a109 = f20(v8.clone());
                let a110 = 1u64;
                nat_add(a109, a110)?
            };
            if nat_lt(a111, a112) {
                f23(f23(v0, v1, v2, v4)?, v5, v6, v8)
            } else {
                let m113 = v4;
                match m113 {
                    Adt0::C0 => {
                        f23(v0, v1, v2, v3)
                    }
                    Adt0::C1(r114, v10, v11, _, r115) => {
                        let v9 = (*r114).clone();
                        let v13 = (*r115).clone();
                        f23(f23(v0, v1, v2, v9)?, v10, v11, f23(v13, v5, v6, v8)?)
                    }
                }
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Bytes, v2: Bytes, v3: Adt0) -> R<Adt0> {
    Ok(Adt0::C1(std::rc::Rc::new(v0.clone()), v1, v2, {
        let a118 = f9(f20(v0), f20(v3.clone()));
        let a119 = 1u64;
        nat_add(a118, a119)?
    }, std::rc::Rc::new(v3)))
}

pub fn f24(v0: Adt0) -> R<(Option<(Bytes, Bytes)>, Adt0)> {
    let m120 = v0;
    match m120 {
        Adt0::C0 => {
            Ok((None::<(Bytes, Bytes)>, Adt0::C0))
        }
        Adt0::C1(r124, v2, v3, _, r125) => {
            let v1 = (*r124).clone();
            let v5 = (*r125).clone();
            let m121 = v1.clone();
            match m121 {
                Adt0::C0 => {
                    Ok((Some((v2, v3)), v5))
                }
                Adt0::C1(_, _, _, _, _) => {
                    let v11: (Option<(Bytes, Bytes)>, Adt0) = f24(v1)?;
                    Ok(({
                        let (h122, _) = v11;
                        h122
                    }, f18({
                        let (_, h123) = v11;
                        h123
                    }, v2, v3, v5)?))
                }
            }
        }
    }
}

pub fn run(p0: List<(u64, (Bytes, Bytes))>) -> R<bool> {
    f0(p0)
}
