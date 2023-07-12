import builtins
from typing import TypeVar, TypeVarTuple, Generic, overload, Protocol

import _cerialize

T = TypeVar("T")
Shape = TypeVarTuple("Shape")


class SupportsBool(Protocol):
    def __bool__(self) -> builtins.bool:
        ...


class bool(Generic[*Shape], _cerialize._bool):
    def __new__(cls, *args) -> "bool":
        value = next(iter(args), builtins.bool())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: "bool") -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = bool(__value.__bool__()) if not isinstance(__value, bool) else __value
        return super().__ge__(value)


class i8(Generic[*Shape], _cerialize._i8):
    def __new__(cls, *args) -> "i8":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "i8") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = i8(int(__value)) if not isinstance(__value, i8) else __value
        return super().__ge__(value)


class i16(Generic[*Shape], _cerialize._i16):
    def __new__(cls, *args) -> "i16":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "i16") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = i16(int(__value)) if not isinstance(__value, i16) else __value
        return super().__ge__(value)


class i32(Generic[*Shape], _cerialize._i32):
    def __new__(cls, *args) -> "i32":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "i32") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = i32(int(__value)) if not isinstance(__value, i32) else __value
        return super().__ge__(value)


class i64(Generic[*Shape], _cerialize._i64):
    def __new__(cls, *args) -> "i64":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "i64") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = i64(int(__value)) if not isinstance(__value, i64) else __value
        return super().__ge__(value)


class u8(Generic[*Shape], _cerialize._u8):
    def __new__(cls, *args) -> "u8":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "u8") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = u8(int(__value)) if not isinstance(__value, u8) else __value
        return super().__ge__(value)


class u16(Generic[*Shape], _cerialize._u16):
    def __new__(cls, *args) -> "u16":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "u16") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = u16(int(__value)) if not isinstance(__value, u16) else __value
        return super().__ge__(value)


class u32(Generic[*Shape], _cerialize._u32):
    def __new__(cls, *args) -> "u32":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "u32") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = u32(int(__value)) if not isinstance(__value, u32) else __value
        return super().__ge__(value)


class u64(Generic[*Shape], _cerialize._u64):
    def __new__(cls, *args) -> "u64":
        value = next(iter(args), int())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "u64") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: int) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = u64(int(__value)) if not isinstance(__value, u64) else __value
        return super().__ge__(value)


class f16(Generic[*Shape], _cerialize._f16):
    def __new__(cls, *args) -> "f16":
        value = next(iter(args), float())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "f16") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: float) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = f16(float(__value)) if not isinstance(__value, f16) else __value
        return super().__ge__(value)


class f32(Generic[*Shape], _cerialize._f32):
    def __new__(cls, *args) -> "f32":
        value = next(iter(args), float())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "f32") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: float) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = f32(float(__value)) if not isinstance(__value, f32) else __value
        return super().__ge__(value)


class f64(Generic[*Shape], _cerialize._f64):
    def __new__(cls, *args) -> "f64":
        value = next(iter(args), float())
        return super().__new__(cls, value)

    @overload
    def __eq__(self, value: "f64") -> builtins.bool:
        ...

    @overload
    def __eq__(self, value: float) -> builtins.bool:
        ...

    def __eq__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__eq__(value)

    @overload
    def __ne__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ne__(self, value: "bool") -> builtins.bool:
        ...

    def __ne__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__ne__(value)

    @overload
    def __lt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __lt__(self, value: "bool") -> builtins.bool:
        ...

    def __lt__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__lt__(value)

    @overload
    def __le__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __le__(self, value: "bool") -> builtins.bool:
        ...

    def __le__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__le__(value)

    @overload
    def __gt__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __gt__(self, value: "bool") -> builtins.bool:
        ...

    def __gt__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__gt__(value)

    @overload
    def __ge__(self, value: builtins.bool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: SupportsBool) -> builtins.bool:
        ...

    @overload
    def __ge__(self, value: "bool") -> builtins.bool:
        ...

    def __ge__(self, __value) -> builtins.bool:
        value = f64(float(__value)) if not isinstance(__value, f64) else __value
        return super().__ge__(value)


class const(Generic[T]):
    pass
