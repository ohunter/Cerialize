import pytest
import cerialize

def test_declare_signed_integer_fields():
    class test_i8():
        _: cerialize.u8
    
    class test_i16():
        _: cerialize.i16

    class test_i32():
        _: cerialize.i32

    class test_i64():
        _: cerialize.i64

    cerialize.cstruct(test_i8)
    cerialize.cstruct(test_i16)
    cerialize.cstruct(test_i32)
    cerialize.cstruct(test_i64)

def test_declare_unsigned_integer_fields():
    class test_u8():
        _: cerialize.u8
    
    class test_u16():
        _: cerialize.u16

    class test_u32():
        _: cerialize.u32

    class test_u64():
        _: cerialize.u64

    cerialize.cstruct(test_u8)
    cerialize.cstruct(test_u16)
    cerialize.cstruct(test_u32)
    cerialize.cstruct(test_u64)

def test_declare_float_fields():
    class test_f16():
        _: cerialize.f16

    class test_f32():
        _: cerialize.f32

    class test_f64():
        _: cerialize.f64

    cerialize.cstruct(test_f16)
    cerialize.cstruct(test_f32)
    cerialize.cstruct(test_f64)

def test_declare_array_fields():
    class test_i8_array():
        _: cerialize.u8[128]

    class test_i16_array():
        _: cerialize.i16[128]

    class test_i32_array():
        _: cerialize.i32[128]

    class test_i64_array():
        _: cerialize.i64[128]

    class test_u8_array():
        _: cerialize.u8[128]

    class test_u16_array():
        _: cerialize.u16[128]

    class test_u32_array():
        _: cerialize.u32[128]

    class test_u64_array():
        _: cerialize.u64[128]

    class test_f16_array():
        _: cerialize.f16[128]

    class test_f32_array():
        _: cerialize.f32[128]

    class test_f64_array():
        _: cerialize.f64[128]
    
    cerialize.cstruct(test_i8_array)
    cerialize.cstruct(test_i16_array)
    cerialize.cstruct(test_i32_array)
    cerialize.cstruct(test_i64_array)
    cerialize.cstruct(test_u8_array)
    cerialize.cstruct(test_u16_array)
    cerialize.cstruct(test_u32_array)
    cerialize.cstruct(test_u64_array)
    cerialize.cstruct(test_f16_array)
    cerialize.cstruct(test_f32_array)
    cerialize.cstruct(test_f64_array)
