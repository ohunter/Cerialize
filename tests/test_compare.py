import pytest
import cerialize


def test_initialize_signed_integer():
    # assert cerialize.i8() == 0
    assert cerialize.i8() == cerialize.i8()
    # assert cerialize.i8(0) == 0
    assert cerialize.i8(0) == cerialize.i8(0)
    # assert cerialize.i8((2**7)-1) == (2**7)-1
    assert cerialize.i8((2**7) - 1) == cerialize.i8((2**7) - 1)

    # assert cerialize.i16() == 0
    assert cerialize.i16() == cerialize.i16()
    # assert cerialize.i16(0) == 0
    assert cerialize.i16(0) == cerialize.i16(0)
    # assert cerialize.i16((2**15)-1) == (2**15)-1
    assert cerialize.i16((2**15) - 1) == cerialize.i16((2**15) - 1)

    # assert cerialize.i32() == 0
    assert cerialize.i32() == cerialize.i32()
    # assert cerialize.i32(0) == 0
    assert cerialize.i32(0) == cerialize.i32(0)
    # assert cerialize.i32((2**31)-1) == (2**31)-1
    assert cerialize.i32((2**31) - 1) == cerialize.i32((2**31) - 1)

    # assert cerialize.i64() == 0
    assert cerialize.i64() == cerialize.i64()
    # assert cerialize.i64(0) == 0
    assert cerialize.i64(0) == cerialize.i64(0)
    # assert cerialize.i64((2**63)-1) == (2**63)-1
    assert cerialize.i64((2**63) - 1) == cerialize.i64((2**63) - 1)


def test_initialize_unsigned_integer():
    cerialize.u8()
    cerialize.u8(0)
    cerialize.u8((2**8) - 1)

    cerialize.u16()
    cerialize.u16(0)
    cerialize.u16((2**16) - 1)

    cerialize.u32()
    cerialize.u32(0)
    cerialize.u32((2**32) - 1)

    cerialize.u64()
    cerialize.u64(0)
    cerialize.u64((2**64) - 1)


def test_initialize_float():
    cerialize.f16()
    cerialize.f16(0)
    cerialize.f16(3.141592653589793)

    cerialize.f32()
    cerialize.f32(0)
    cerialize.f32(3.141592653589793)

    cerialize.f64()
    cerialize.f64(0)
    cerialize.f64(3.141592653589793)


# def test_initialize_array_fields():
#     class i8_array:
#         _: cerialize.u8[128]

#     class i16_array:
#         _: cerialize.i16[128]

#     class i32_array:
#         _: cerialize.i32[128]

#     class i64_array:
#         _: cerialize.i64[128]

#     class u8_array:
#         _: cerialize.u8[128]

#     class u16_array:
#         _: cerialize.u16[128]

#     class u32_array:
#         _: cerialize.u32[128]

#     class u64_array:
#         _: cerialize.u64[128]

#     class f16_array:
#         _: cerialize.f16[128]

#     class f32_array:
#         _: cerialize.f32[128]

#     class f64_array:
#         _: cerialize.f64[128]

#     cerialize.cstruct(i8_array)
#     cerialize.cstruct(i16_array)
#     cerialize.cstruct(i32_array)
#     cerialize.cstruct(i64_array)
#     cerialize.cstruct(u8_array)
#     cerialize.cstruct(u16_array)
#     cerialize.cstruct(u32_array)
#     cerialize.cstruct(u64_array)
#     cerialize.cstruct(f16_array)
#     cerialize.cstruct(f32_array)
#     cerialize.cstruct(f64_array)


# def test_initialize_const_field():
#     class const_i8:
#         _: cerialize.const[cerialize.u8]

#     class const_i16:
#         _: cerialize.const[cerialize.i16]

#     class const_i32:
#         _: cerialize.const[cerialize.i32]

#     class const_i64:
#         _: cerialize.const[cerialize.i64]

#     class const_u8:
#         _: cerialize.const[cerialize.u8]

#     class const_u16:
#         _: cerialize.const[cerialize.u16]

#     class const_u32:
#         _: cerialize.const[cerialize.u32]

#     class const_u64:
#         _: cerialize.const[cerialize.u64]

#     class const_f16:
#         _: cerialize.const[cerialize.f16]

#     class const_f32:
#         _: cerialize.const[cerialize.f32]

#     class const_f64:
#         _: cerialize.const[cerialize.f64]

#     cerialize.cstruct(const_i8)
#     cerialize.cstruct(const_i16)
#     cerialize.cstruct(const_i32)
#     cerialize.cstruct(const_i64)
#     cerialize.cstruct(const_u8)
#     cerialize.cstruct(const_u16)
#     cerialize.cstruct(const_u32)
#     cerialize.cstruct(const_u64)
#     cerialize.cstruct(const_f16)
#     cerialize.cstruct(const_f32)
#     cerialize.cstruct(const_f64)


# def test_initialize_const_array_field():
#     class const_i8_array:
#         _: cerialize.const[cerialize.u8[128]]

#     class const_i16_array:
#         _: cerialize.const[cerialize.i16[128]]

#     class const_i32_array:
#         _: cerialize.const[cerialize.i32[128]]

#     class const_i64_array:
#         _: cerialize.const[cerialize.i64[128]]

#     class const_u8_array:
#         _: cerialize.const[cerialize.u8[128]]

#     class const_u16_array:
#         _: cerialize.const[cerialize.u16[128]]

#     class const_u32_array:
#         _: cerialize.const[cerialize.u32[128]]

#     class const_u64_array:
#         _: cerialize.const[cerialize.u64[128]]

#     class const_f16_array:
#         _: cerialize.const[cerialize.f16[128]]

#     class const_f32_array:
#         _: cerialize.const[cerialize.f32[128]]

#     class const_f64_array:
#         _: cerialize.const[cerialize.f64[128]]

#     cerialize.cstruct(const_i8_array)
#     cerialize.cstruct(const_i16_array)
#     cerialize.cstruct(const_i32_array)
#     cerialize.cstruct(const_i64_array)
#     cerialize.cstruct(const_u8_array)
#     cerialize.cstruct(const_u16_array)
#     cerialize.cstruct(const_u32_array)
#     cerialize.cstruct(const_u64_array)
#     cerialize.cstruct(const_f16_array)
#     cerialize.cstruct(const_f32_array)
#     cerialize.cstruct(const_f64_array)


# def test_initialize_nested_field():
#     class nesting0:
#         _: cerialize.i8

#     class nesting1:
#         _: cerialize.cstruct(nesting0)

#     class nesting2:
#         _: cerialize.cstruct(nesting1)

#     cerialize.cstruct(nesting2)
