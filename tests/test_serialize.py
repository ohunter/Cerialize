import pytest
import cerialize
from typing import TypeVarTuple, Generic

Shape = TypeVarTuple("Shape")


def test_serialize_bool():
    @cerialize.cstruct
    class bool_test:
        _: cerialize.bool

    x = bool_test(True)
