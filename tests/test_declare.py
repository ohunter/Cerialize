import pytest
import cerialize
from typing import TypeVarTuple, Generic

Shape = TypeVarTuple("Shape")


def test_declare_bool_fields():
    class bool:
        _: cerialize.bool

    cerialize.cstruct(bool)


def test_declare_signed_integer_fields():
    class i8:
        _: cerialize.i8

    class i16:
        _: cerialize.i16

    class i32:
        _: cerialize.i32

    class i64:
        _: cerialize.i64

    cerialize.cstruct(i8)
    cerialize.cstruct(i16)
    cerialize.cstruct(i32)
    cerialize.cstruct(i64)


def test_declare_unsigned_integer_fields():
    class u8:
        _: cerialize.u8

    class u16:
        _: cerialize.u16

    class u32:
        _: cerialize.u32

    class u64:
        _: cerialize.u64

    cerialize.cstruct(u8)
    cerialize.cstruct(u16)
    cerialize.cstruct(u32)
    cerialize.cstruct(u64)


def test_declare_float_fields():
    class f16:
        _: cerialize.f16

    class f32:
        _: cerialize.f32

    class f64:
        _: cerialize.f64

    cerialize.cstruct(f16)
    cerialize.cstruct(f32)
    cerialize.cstruct(f64)


def test_declare_array_fields():
    class bool_array:
        _: cerialize.bool[128]

    class i8_array:
        _: cerialize.i8[128]

    class i16_array:
        _: cerialize.i16[128]

    class i32_array:
        _: cerialize.i32[128]

    class i64_array:
        _: cerialize.i64[128]

    class u8_array:
        _: cerialize.u8[128]

    class u16_array:
        _: cerialize.u16[128]

    class u32_array:
        _: cerialize.u32[128]

    class u64_array:
        _: cerialize.u64[128]

    class f16_array:
        _: cerialize.f16[128]

    class f32_array:
        _: cerialize.f32[128]

    class f64_array:
        _: cerialize.f64[128]

    cerialize.cstruct(bool_array)
    cerialize.cstruct(i8_array)
    cerialize.cstruct(i16_array)
    cerialize.cstruct(i32_array)
    cerialize.cstruct(i64_array)
    cerialize.cstruct(u8_array)
    cerialize.cstruct(u16_array)
    cerialize.cstruct(u32_array)
    cerialize.cstruct(u64_array)
    cerialize.cstruct(f16_array)
    cerialize.cstruct(f32_array)
    cerialize.cstruct(f64_array)

def test_declare_nested_field():
    class inner(Generic[*Shape]):
        _: cerialize.i8

    class nesting_single:
        _: cerialize.cstruct(inner)

    class nesting_multiple:
        a: cerialize.cstruct(inner)
        b: cerialize.cstruct(inner)

    class nesting_1d_array:
        _: cerialize.cstruct(inner[10])

    class nesting_2d_array:
        _: cerialize.cstruct(inner[10, 10])

    cerialize.cstruct(nesting_single)
    cerialize.cstruct(nesting_multiple)
    cerialize.cstruct(nesting_1d_array)
    cerialize.cstruct(nesting_2d_array)


def test_declare_empty_struct():
    class empty:
        pass

    cerialize.cstruct(empty)
