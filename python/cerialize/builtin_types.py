from typing import TypeVar, TypeVarTuple, Generic

# from _cerialize import *
import _cerialize

T = TypeVar("T")
Shape = TypeVarTuple("Shape")


class i8(Generic[*Shape], _cerialize._i8):
    pass


class i16(Generic[*Shape], _cerialize._i16):
    pass


class i32(Generic[*Shape], _cerialize._i32):
    pass


class i64(Generic[*Shape], _cerialize._i64):
    pass


class u8(Generic[*Shape], _cerialize._u8):
    pass


class u16(Generic[*Shape], _cerialize._u16):
    pass


class u32(Generic[*Shape], _cerialize._u32):
    pass


class u64(Generic[*Shape], _cerialize._u64):
    pass


class f16(Generic[*Shape], _cerialize._f16):
    pass


class f32(Generic[*Shape], _cerialize._f32):
    pass


class f64(Generic[*Shape], _cerialize._f64):
    pass


class const(Generic[T]):
    pass
