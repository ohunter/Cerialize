from enum import Enum
from math import prod
import sys

from .builtin_types import *

class endianness(Enum):
    native = 0
    little = 1
    big = 2

def _process_class(cls: type, endianness: endianness , alignment: int , packed: bool, serialize: bool, deserialize: bool):
    # This function is heavily based on how dataclasses' solve the issue of type introspection

    # Dictionaries have ordered insertion which comes to play here and does have an effect on the fields themselves
    fields = {}

    if cls.__module__ in sys.modules:
        globals = sys.modules[cls.__module__].__dict__
    else:
        globals = {}
    __ignored_attributes = {'__module__', '__annotations__', '__dict__', '__weakref__', '__doc__'}
    annotations = cls.__dict__.get("__annotations__", {})

    # Figure out if there are any fields which aren't type annotated properly
    for name, value in [(x,y) for x,y in cls.__dict__.items() if x not in __ignored_attributes]:
        if name not in annotations and not isinstance(value, type):
            raise TypeError(f"Field {name!r} in {cls!r} is missing a type annotation")

    # Things that are done here:
    # - Check if the annotation is supported
    # - Generate appropriate sizes based on the annotations (and their possible size arguments in '__args__')
    for name, type in annotations.items():
        # shape = type.__dict__.get("__args__", (1,))
        # size = type._size * prod(shape)
        # print(shape, size)
        breakpoint()

    # TODO: Generate getters and delete setters for constant fields
    # 

def cstruct(cls: type | None =None, /, *, endianness: endianness =endianness.native, alignment: int =1, packed: bool=False, serialize: bool=False, deserialize: bool=False):
    """
    Uses PEP 526's __annotations__ to extract the type
    """
    def wrap(cls):
        return _process_class(cls, endianness, alignment, packed, serialize, deserialize)
    
    # Allows for use by both @cstruct and cstruct()
    return wrap if cls is None else wrap(cls)
