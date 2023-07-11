import pytest
import cerialize


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


def test_declare_const_field():
    class const_bool:
        _: cerialize.const[cerialize.bool]

    class const_i8:
        _: cerialize.const[cerialize.i8]

    class const_i16:
        _: cerialize.const[cerialize.i16]

    class const_i32:
        _: cerialize.const[cerialize.i32]

    class const_i64:
        _: cerialize.const[cerialize.i64]

    class const_u8:
        _: cerialize.const[cerialize.u8]

    class const_u16:
        _: cerialize.const[cerialize.u16]

    class const_u32:
        _: cerialize.const[cerialize.u32]

    class const_u64:
        _: cerialize.const[cerialize.u64]

    class const_f16:
        _: cerialize.const[cerialize.f16]

    class const_f32:
        _: cerialize.const[cerialize.f32]

    class const_f64:
        _: cerialize.const[cerialize.f64]

    cerialize.cstruct(const_bool)
    cerialize.cstruct(const_i8)
    cerialize.cstruct(const_i16)
    cerialize.cstruct(const_i32)
    cerialize.cstruct(const_i64)
    cerialize.cstruct(const_u8)
    cerialize.cstruct(const_u16)
    cerialize.cstruct(const_u32)
    cerialize.cstruct(const_u64)
    cerialize.cstruct(const_f16)
    cerialize.cstruct(const_f32)
    cerialize.cstruct(const_f64)


def test_declare_const_array_field():
    class const_bool_array:
        _: cerialize.const[cerialize.bool[128]]

    class const_i8_array:
        _: cerialize.const[cerialize.i8[128]]

    class const_i16_array:
        _: cerialize.const[cerialize.i16[128]]

    class const_i32_array:
        _: cerialize.const[cerialize.i32[128]]

    class const_i64_array:
        _: cerialize.const[cerialize.i64[128]]

    class const_u8_array:
        _: cerialize.const[cerialize.u8[128]]

    class const_u16_array:
        _: cerialize.const[cerialize.u16[128]]

    class const_u32_array:
        _: cerialize.const[cerialize.u32[128]]

    class const_u64_array:
        _: cerialize.const[cerialize.u64[128]]

    class const_f16_array:
        _: cerialize.const[cerialize.f16[128]]

    class const_f32_array:
        _: cerialize.const[cerialize.f32[128]]

    class const_f64_array:
        _: cerialize.const[cerialize.f64[128]]

    cerialize.cstruct(const_bool_array)
    cerialize.cstruct(const_i8_array)
    cerialize.cstruct(const_i16_array)
    cerialize.cstruct(const_i32_array)
    cerialize.cstruct(const_i64_array)
    cerialize.cstruct(const_u8_array)
    cerialize.cstruct(const_u16_array)
    cerialize.cstruct(const_u32_array)
    cerialize.cstruct(const_u64_array)
    cerialize.cstruct(const_f16_array)
    cerialize.cstruct(const_f32_array)
    cerialize.cstruct(const_f64_array)


def test_declare_nested_field():
    class nesting0:
        _: cerialize.i8

    class nesting1:
        _: cerialize.cstruct(nesting0)

    class nesting2:
        _: cerialize.cstruct(nesting1)

    cerialize.cstruct(nesting2)
