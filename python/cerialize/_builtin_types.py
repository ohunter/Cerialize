from typing import TypeVar, TypeVarTuple, Generic

T = TypeVar('T')
Shape = TypeVarTuple("Shape")

class i8(Generic[*Shape]):
    _size: int = 1
    pass

class i16(Generic[*Shape]):
    _size: int = 2
    pass

class i32(Generic[*Shape]):
    _size: int = 3
    pass

class i64(Generic[*Shape]):
    _size: int = 4
    pass

class u8(Generic[*Shape]):
    _size: int = 1
    pass

class u16(Generic[*Shape]):
    _size: int = 2
    pass

class u32(Generic[*Shape]):
    _size: int = 3
    pass

class u64(Generic[*Shape]):
    _size: int = 4
    pass

class f16(Generic[*Shape]):
    _size: int = 2
    pass

class f32(Generic[*Shape]):
    _size: int = 3
    pass

class f64(Generic[*Shape]):
    _size: int = 4
    pass

class const(Generic[T]):
    # Size is determined from the generic argument
    pass