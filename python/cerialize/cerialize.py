from enum import Enum
from typing import _GenericAlias, Any, Generic, TypeVarTuple
from types import NoneType, new_class
from functools import lru_cache
from math import prod
from dataclasses import dataclass

import cerialize.base_types as basic

Shape = TypeVarTuple("Shape")

@dataclass(eq=True, frozen=True)
class _type_spesification:
    base: type
    shape: tuple[int, ...]

@dataclass
class _func_arg:
    name: str
    type: type
    prefix: str = ""

class endianness(Enum):
    native = 0
    little = 1
    big = 2

def _get_properties(cls: type) -> dict[str, Any]:
    if isinstance(cls, _GenericAlias):
        return cls.__origin__.__dict__

    return cls.__dict__


def _create_fn(
    name: str,
    args: list[_func_arg] = [],
    return_type: type = NoneType,
    body: list[str] = ["pass"],
    globals: dict[str, Any] | None = None,
    locals: dict[str, Any] | None = None,
):
    # This copied directly from dataclasses as I figured it'd work here as well
    # Minor modifications have been made based on some assumptions I have made

    if locals is None:
        locals = {}

    locals.update({f"_{arg.name}_type": arg.type for arg in args})
    locals["_return_type"] = return_type
    return_annotation = "-> _return_type"

    arg_txt = ", ".join([f"{arg.prefix}{arg.name}: _{arg.name}_type" for arg in args])
    body = "\n".join(f"  {b}" for b in body)

    # Compute the text of the entire function.
    txt = f" def {name}({arg_txt}){return_annotation}:\n{body}"

    local_vars = ", ".join(locals.keys())
    txt = f"def __create_fn__({local_vars}):\n{txt}\n return {name}"
    ns = {}
    exec(txt, globals, ns)
    return ns["__create_fn__"](**locals)

def _define_init(cls: type, *args: _func_arg, body: list[str] | None = None) -> None:
    args = list(*args)
    if body is None:
        body = [
            f"self.{arg.name} = {arg.name}" for arg in args
        ] + ["pass"]
    init_fn = _create_fn("__init__", [_func_arg(name="self", type=cls)] + args, cls, body)
    init_fn.__qualname__ = f"{cls.__qualname__}.{init_fn.__name__}"
    setattr(cls, "__init__", init_fn)

def _define_repr(cls: type) -> None:
    body = [
        f"return self.__class__.__qualname__ + f'("+ ', '.join([f"{attr}={{self.{attr}!r}}" for attr in cls._CFIELDS.keys()]) + ")'"
    ]
    repr_fn = _create_fn("__repr__", [_func_arg(name="self", type=cls)], str, body)
    repr_fn.__qualname__ = f"{cls.__qualname__}.{repr_fn.__name__}"
    setattr(cls, "__repr__", repr_fn)

def _supported_type(cls: type) -> bool:
    __baseline_types = {
        basic.bool,
        basic.i8,
        basic.i16,
        basic.i32,
        basic.i64,
        basic.u8,
        basic.u16,
        basic.u32,
        basic.u64,
        basic.f16,
        basic.f32,
        basic.f64,
    }

    if cls in __baseline_types:
        return True

    fields: dict[str, _type_spesification] | None = cls.__dict__.get("_CFIELDS")

    # I don't think checking if this is None is the best idea, but it seems to fix an issue where the `_CFIELDS` class attribute is overwritten for some reason
    if fields is not None:
        # Check that all the internal fields are supported
        return all(_supported_type(v) for v in fields.values())

    breakpoint()
    return False


def _determine_type(cls: type, shape: tuple[int, ...] | None = None) -> _type_spesification:
    # Check if type has nested arguments
    match cls.__dict__.get("__args__"):
        case None:
            # The type is the base case
            return _type_spesification(base=cls, shape=shape if shape is not None else (1,))
        case [*vals] if all(isinstance(x, int) for x in vals):
            # The type has some dimensions that need to be considered
            if shape is not None:
                raise ValueError(f"Attempt to override assigned shape")
            return _determine_type(cls.__origin__, tuple(vals))
        case _:
            # Uh oh... Something has gone wrong
            raise NotImplementedError(f"Unable to determine support for type {cls!r}")

@lru_cache
def _resolve_type(spec: _type_spesification) -> type:
    # Determine the actual type that should be used for this field generating a new type if neccessary

    match spec.shape:
        case (1,):
            return spec.base
        case [*shape]:
            # Create a new type which is based on the base type but with `prod(*shape)` elements
            new_type = type(f"{spec.base.__name__}[{','.join(f'{x!s}' for x in shape)}]",
                            (spec.base,),
                            {
                                "__module__": spec.base.__module__,
                            }
            )

            # The new type's constructor will accept one item or `prod(*shape)` items
            # If only one item is provided, that value is assigned to every index
            # If `prod(*shape)` items are provided, each index gets its own value
            init_body = [
                f"match args:",
                f"  case (value,):",
                f"    self._data = [value] * {prod(shape)}",
                f"  case [*values] if len(values) == {prod(shape)}:",
                f"    self._data = list(*values)",
                f"  case _:",
                f"    raise Exception('Invalid number of arguments')",
            ]
            init_args = [_func_arg(name="args", type=spec.base, prefix="*")]
            _define_init(new_type, init_args, body=init_body)
            return new_type
        case _:
            # Uh oh... Something has gone wrong
            raise NotImplementedError(f"Unable to resolve type for {spec!s}")


def _process_class(
    cls: type,
    generate_init: bool,
    generate_repr: bool,
    endianness: endianness,
    alignment: int,
    packed: bool,
    serialize: bool,
    deserialize: bool,
):
    # This function is heavily based on how dataclasses' solve the issue of type introspection
    __ignored_attributes = {
        # Basic python attributes
        "__annotations__",
        "__args__",
        "__dict__",
        "__doc__",
        "__init__",
        "__module__",
        "__orig_bases__",
        "__origin__",
        "__parameters__",
        "__parameters__",
        "__slots__",
        "__weakref__",
        # Cerialized class attributes
        "_CFIELDS",
    }

    # Dictionaries have ordered insertion which comes to play here and does have an effect on the fields themselves
    fields: dict[str, _type_spesification] = {}
    annotations = cls.__dict__.get("__annotations__", {})

    # Figure out if there are any fields which aren't type annotated properly
    for name, value in _get_properties(cls).items():
        if name in __ignored_attributes:
            continue
        elif name not in annotations and not isinstance(value, (type, _GenericAlias)):
            breakpoint()
            raise TypeError(f"Field {name!r} in {cls!r} is missing a type annotation")


    # Check if the annotation is supported
    for name, _type in annotations.items():
        _type_spec = _determine_type(_type)
        if not _supported_type(_type_spec.base):
            breakpoint()
            raise TypeError(
                f"Field {name!r} in {cls!r} is annotated with an unsupported type"
            )
        else:
            fields.update({name: _resolve_type(_type_spec)})

    new_type = new_class(cls.__name__, (cls, Generic[*Shape]))
    setattr(new_type, "__module__", cls.__module__)
    setattr(new_type, "__annotations__", fields)
    setattr(new_type, "_CFIELDS", fields)

    # TODO: Figure out which fields have initializers
    initialized: set[str] = set()

    # TODO: Generate getters and delete setters for constant fields

    # Generate an `__init__` function if `generate_init` is set and it isn't already defined
    if generate_init and "__init__" not in cls.__dict__:
        _define_init(new_type, (_func_arg(name=name, type=_type) for name, _type in fields.items() if name not in initialized))

    if generate_repr and "__repr__" not in cls.__dict__:
        _define_repr(new_type)

    return new_type


def cstruct(
    cls: type | None = None,
    /,
    *,
    init: bool = True,
    repr: bool = True,
    endianness: endianness = endianness.native,
    alignment: int = 1,
    packed: bool = False,
    serialize: bool = False,
    deserialize: bool = False,
):
    """
    Uses PEP 526's __annotations__ to extract the type
    """

    def wrap(cls):
        return _process_class(
            cls, init, repr, endianness, alignment, packed, serialize, deserialize
        )

    # Allows for use by both @cstruct and cstruct()
    return wrap if cls is None else wrap(cls)
