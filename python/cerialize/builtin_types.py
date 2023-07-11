import builtins
from typing import TypeVar, TypeVarTuple, Generic

import _cerialize

T = TypeVar("T")
Shape = TypeVarTuple("Shape")


class bool(Generic[*Shape], _cerialize._bool):
    def __new__(cls, *args) -> "bool":
        value = next(iter(args), builtins.bool())
        return super().__new__(cls, value)


class i8(Generic[*Shape], _cerialize._i8):
    def __new__(cls, *args) -> "i8":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class i16(Generic[*Shape], _cerialize._i16):
    def __new__(cls, *args) -> "i16":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class i32(Generic[*Shape], _cerialize._i32):
    def __new__(cls, *args) -> "i32":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class i64(Generic[*Shape], _cerialize._i64):
    def __new__(cls, *args) -> "i64":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class u8(Generic[*Shape], _cerialize._u8):
    def __new__(cls, *args) -> "u8":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class u16(Generic[*Shape], _cerialize._u16):
    def __new__(cls, *args) -> "u16":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class u32(Generic[*Shape], _cerialize._u32):
    def __new__(cls, *args) -> "u32":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class u64(Generic[*Shape], _cerialize._u64):
    def __new__(cls, *args) -> "u64":
        value = next(iter(args), int())
        return super().__new__(cls, value)


class f16(Generic[*Shape], _cerialize._f16):
    def __new__(cls, *args) -> "f16":
        value = next(iter(args), float())
        return super().__new__(cls, value)


class f32(Generic[*Shape], _cerialize._f32):
    def __new__(cls, *args) -> "f32":
        value = next(iter(args), float())
        return super().__new__(cls, value)


class f64(Generic[*Shape], _cerialize._f64):
    def __new__(cls, *args) -> "f64":
        value = next(iter(args), float())
        return super().__new__(cls, value)


class const(Generic[T]):
    pass
