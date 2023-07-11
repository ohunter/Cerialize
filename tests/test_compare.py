import pytest
import cerialize


def test_compare_signed_integer_equal():
    assert cerialize.i8() == 0
    assert cerialize.i8() == cerialize.i8()
    assert cerialize.i8(0) == 0
    assert cerialize.i8(0) == cerialize.i8(0)
    assert cerialize.i8((2**7) - 1) == (2**7) - 1
    assert cerialize.i8((2**7) - 1) == cerialize.i8((2**7) - 1)

    assert cerialize.i16() == 0
    assert cerialize.i16() == cerialize.i16()
    assert cerialize.i16(0) == 0
    assert cerialize.i16(0) == cerialize.i16(0)
    assert cerialize.i16((2**15) - 1) == (2**15) - 1
    assert cerialize.i16((2**15) - 1) == cerialize.i16((2**15) - 1)

    assert cerialize.i32() == 0
    assert cerialize.i32() == cerialize.i32()
    assert cerialize.i32(0) == 0
    assert cerialize.i32(0) == cerialize.i32(0)
    assert cerialize.i32((2**31) - 1) == (2**31) - 1
    assert cerialize.i32((2**31) - 1) == cerialize.i32((2**31) - 1)

    assert cerialize.i64() == 0
    assert cerialize.i64() == cerialize.i64()
    assert cerialize.i64(0) == 0
    assert cerialize.i64(0) == cerialize.i64(0)
    assert cerialize.i64((2**63) - 1) == (2**63) - 1
    assert cerialize.i64((2**63) - 1) == cerialize.i64((2**63) - 1)


def test_compare_signed_integer_not_equal():
    assert cerialize.i8() != 1
    assert cerialize.i8() != cerialize.i8(1)
    assert cerialize.i8(0) != 1
    assert cerialize.i8(0) != cerialize.i8(1)
    assert cerialize.i8((2**7) - 1) != 0
    assert cerialize.i8((2**7) - 1) != cerialize.i8(0)

    assert cerialize.i16() != 1
    assert cerialize.i16() != cerialize.i16(1)
    assert cerialize.i16(0) != 1
    assert cerialize.i16(0) != cerialize.i16(1)
    assert cerialize.i16((2**15) - 1) != 0
    assert cerialize.i16((2**15) - 1) != cerialize.i16(0)

    assert cerialize.i32() != 1
    assert cerialize.i32() != cerialize.i32(1)
    assert cerialize.i32(0) != 1
    assert cerialize.i32(0) != cerialize.i32(1)
    assert cerialize.i32((2**31) - 1) != 0
    assert cerialize.i32((2**31) - 1) != cerialize.i32(0)

    assert cerialize.i64() != 1
    assert cerialize.i64() != cerialize.i64(1)
    assert cerialize.i64(0) != 1
    assert cerialize.i64(0) != cerialize.i64(1)
    assert cerialize.i64((2**63) - 1) != 0
    assert cerialize.i64((2**63) - 1) != cerialize.i64(0)


def test_compare_signed_integer_less_than():
    assert cerialize.i8() < 1
    assert cerialize.i8() < cerialize.i8(1)
    assert cerialize.i8(0) < 1
    assert cerialize.i8(0) < cerialize.i8(1)

    assert cerialize.i16() < 1
    assert cerialize.i16() < cerialize.i16(1)
    assert cerialize.i16(0) < 1
    assert cerialize.i16(0) < cerialize.i16(1)

    assert cerialize.i32() < 1
    assert cerialize.i32() < cerialize.i32(1)
    assert cerialize.i32(0) < 1
    assert cerialize.i32(0) < cerialize.i32(1)

    assert cerialize.i64() < 1
    assert cerialize.i64() < cerialize.i64(1)
    assert cerialize.i64(0) < 1
    assert cerialize.i64(0) < cerialize.i64(1)


def test_compare_signed_integer_less_than_or_equal():
    assert cerialize.i8() <= 0
    assert cerialize.i8() <= 1
    assert cerialize.i8() <= cerialize.i8(1)
    assert cerialize.i8() <= cerialize.i8(1)
    assert cerialize.i8(0) <= 0
    assert cerialize.i8(0) <= 1
    assert cerialize.i8(0) <= cerialize.i8(0)
    assert cerialize.i8(0) <= cerialize.i8(1)
    assert cerialize.i8((2**7) - 1) <= (2**7) - 1
    assert cerialize.i8((2**7) - 1) <= cerialize.i8((2**7) - 1)

    assert cerialize.i16() <= 0
    assert cerialize.i16() <= 1
    assert cerialize.i16() <= cerialize.i16(1)
    assert cerialize.i16() <= cerialize.i16(1)
    assert cerialize.i16(0) <= 0
    assert cerialize.i16(0) <= 1
    assert cerialize.i16(0) <= cerialize.i16(0)
    assert cerialize.i16(0) <= cerialize.i16(1)
    assert cerialize.i16((2**15) - 1) <= (2**15) - 1
    assert cerialize.i16((2**15) - 1) <= cerialize.i16((2**15) - 1)

    assert cerialize.i32() <= 0
    assert cerialize.i32() <= 1
    assert cerialize.i32() <= cerialize.i32(1)
    assert cerialize.i32() <= cerialize.i32(1)
    assert cerialize.i32(0) <= 0
    assert cerialize.i32(0) <= 1
    assert cerialize.i32(0) <= cerialize.i32(0)
    assert cerialize.i32(0) <= cerialize.i32(1)
    assert cerialize.i32((2**31) - 1) <= (2**31) - 1
    assert cerialize.i32((2**31) - 1) <= cerialize.i32((2**31) - 1)

    assert cerialize.i64() <= 0
    assert cerialize.i64() <= 1
    assert cerialize.i64() <= cerialize.i64(1)
    assert cerialize.i64() <= cerialize.i64(1)
    assert cerialize.i64(0) <= 0
    assert cerialize.i64(0) <= 1
    assert cerialize.i64(0) <= cerialize.i64(0)
    assert cerialize.i64(0) <= cerialize.i64(1)
    assert cerialize.i64((2**63) - 1) <= (2**63) - 1
    assert cerialize.i64((2**63) - 1) <= cerialize.i64((2**63) - 1)


def test_compare_signed_integer_greater_than():
    assert cerialize.i8(1) > 0
    assert cerialize.i8(1) > cerialize.i8(0)

    assert cerialize.i16(1) > 0
    assert cerialize.i16(1) > cerialize.i16(0)

    assert cerialize.i32(1) > 0
    assert cerialize.i32(1) > cerialize.i32(0)

    assert cerialize.i64(1) > 0
    assert cerialize.i64(1) > cerialize.i64(0)


def test_compare_signed_integer_greater_than_or_equal():
    assert cerialize.i8() >= 0
    assert cerialize.i8() >= cerialize.i8()
    assert cerialize.i8(0) >= 0
    assert cerialize.i8(1) >= 0
    assert cerialize.i8(0) >= cerialize.i8(0)
    assert cerialize.i8(1) >= cerialize.i8(0)
    assert cerialize.i8((2**7) - 1) >= (2**7) - 1
    assert cerialize.i8((2**7) - 1) >= cerialize.i8((2**7) - 1)

    assert cerialize.i16() >= 0
    assert cerialize.i16() >= cerialize.i16()
    assert cerialize.i16(0) >= 0
    assert cerialize.i16(1) >= 0
    assert cerialize.i16(0) >= cerialize.i16(0)
    assert cerialize.i16(1) >= cerialize.i16(0)
    assert cerialize.i16((2**15) - 1) >= (2**15) - 1
    assert cerialize.i16((2**15) - 1) >= cerialize.i16((2**15) - 1)

    assert cerialize.i32() >= 0
    assert cerialize.i32() >= cerialize.i32()
    assert cerialize.i32(0) >= 0
    assert cerialize.i32(1) >= 0
    assert cerialize.i32(0) >= cerialize.i32(0)
    assert cerialize.i32(1) >= cerialize.i32(0)
    assert cerialize.i32((2**31) - 1) >= (2**31) - 1
    assert cerialize.i32((2**31) - 1) >= cerialize.i32((2**31) - 1)

    assert cerialize.i64() >= 0
    assert cerialize.i64() >= cerialize.i64()
    assert cerialize.i64(0) >= 0
    assert cerialize.i64(1) >= 0
    assert cerialize.i64(0) >= cerialize.i64(0)
    assert cerialize.i64(1) >= cerialize.i64(0)
    assert cerialize.i64((2**63) - 1) >= (2**63) - 1
    assert cerialize.i64((2**63) - 1) >= cerialize.i64((2**63) - 1)


def test_compare_unsigned_integer():
    assert cerialize.u8() == 0
    assert cerialize.u8() == cerialize.u8()
    assert cerialize.u8(0) == 0
    assert cerialize.u8(0) == cerialize.u8(0)
    assert cerialize.u8((2**8) - 1) == (2**8) - 1
    assert cerialize.u8((2**8) - 1) == cerialize.u8((2**8) - 1)

    assert cerialize.u16() == 0
    assert cerialize.u16() == cerialize.u16()
    assert cerialize.u16(0) == 0
    assert cerialize.u16(0) == cerialize.u16(0)
    assert cerialize.u16((2**16) - 1) == (2**16) - 1
    assert cerialize.u16((2**16) - 1) == cerialize.u16((2**16) - 1)

    assert cerialize.u32() == 0
    assert cerialize.u32() == cerialize.u32()
    assert cerialize.u32(0) == 0
    assert cerialize.u32(0) == cerialize.u32(0)
    assert cerialize.u32((2**32) - 1) == (2**32) - 1
    assert cerialize.u32((2**32) - 1) == cerialize.u32((2**32) - 1)

    assert cerialize.u64() == 0
    assert cerialize.u64() == cerialize.u64()
    assert cerialize.u64(0) == 0
    assert cerialize.u64(0) == cerialize.u64(0)
    assert cerialize.u64((2**64) - 1) == (2**64) - 1
    assert cerialize.u64((2**64) - 1) == cerialize.u64((2**64) - 1)


def test_compare_float():
    # NOTE: Comparing floats with `==` is generally a bad idea...
    # Should probably look into a different way of doing this
    assert cerialize.f16() == 0
    assert cerialize.f16() == cerialize.f16()
    assert cerialize.f16(0) == 0
    assert cerialize.f16(0) == cerialize.f16(0)
    assert cerialize.f16(3.141592653589793) == 3.140625
    assert cerialize.f16(3.141592653589793) == cerialize.f16(3.141592653589793)

    assert cerialize.f32() == 0
    assert cerialize.f32() == cerialize.f32()
    assert cerialize.f32(0) == 0
    assert cerialize.f32(0) == cerialize.f32(0)
    assert cerialize.f32(3.141592653589793) == 3.1415927
    assert cerialize.f32(3.141592653589793) == cerialize.f32(3.141592653589793)

    assert cerialize.f64() == 0
    assert cerialize.f64() == cerialize.f64()
    assert cerialize.f64(0) == 0
    assert cerialize.f64(0) == cerialize.f64(0)
    assert cerialize.f64(3.141592653589793) == 3.141592653589793
    assert cerialize.f64(3.141592653589793) == cerialize.f64(3.141592653589793)
