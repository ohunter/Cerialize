from enum import Enum
from typing import TypedDict, _GenericAlias, Any
from types import NoneType

import cerialize.builtin_types as builtin


class _type_spesification(TypedDict):
    base: type
    shape: tuple[int] | None
    modifiers: set[builtin.const]


class endianness(Enum):
    native = 0
    little = 1
    big = 2


def _supported_type(cls: type) -> bool:
    __baseline_types = {
        builtin.bool,
        builtin.i8,
        builtin.i16,
        builtin.i32,
        builtin.i64,
        builtin.u8,
        builtin.u16,
        builtin.u32,
        builtin.u64,
        builtin.f16,
        builtin.f32,
        builtin.f64,
    }

    if cls in __baseline_types:
        return True

    fields: dict[str, _type_spesification] | None = cls.__dict__.get("_CFIELDS")

    # I don't think checking if this is None is the best idea, but it seems to fix an issue where the `_CFIELDS` class attribute is overwritten for some reason
    if fields is not None:
        # Check that all the internal fields are supported
        return all(_supported_type(v["base"]) for v in fields.values())

    breakpoint()
    return False


def _determine_type(cls: type) -> _type_spesification:
    # Check if type has nested arguments
    match cls.__dict__.get("__args__"):
        case None:
            # The type is the base case
            return _type_spesification(base=cls)
        case (_type,) if isinstance(_type, (type, _GenericAlias)):
            # The type wraps another type which may also wrap something else
            spec = _determine_type(_type)
            modifiers = spec.get("modifiers", set())
            modifiers.add(
                cls().__class__
            )  # This is not ideal if the class has an expensive default constructor
            spec["modifiers"] = modifiers
            return spec
        case [*vals] if all(isinstance(x, int) for x in vals):
            # The type has some dimensions that need to be considered
            spec = _determine_type(
                cls().__class__
            )  # This is not ideal if the class has an expensive default constructor
            spec["shape"] = tuple(vals)
            return spec
        case value:
            # Uh oh... Something has gone wrong
            raise NotImplementedError(f"Unable to determine support for type {cls!r}")


def _get_properties(cls: type) -> dict[str, Any]:
    if isinstance(cls, _GenericAlias):
        return cls.__origin__.__dict__

    return cls.__dict__


def _create_fn(
    name: str,
    args: list[tuple[str, type]] = [],
    return_type: type = NoneType,
    body: list[str] = ["pass"],
    globals: dict[str, Any] | None = None,
    locals: dict[str, Any] | None = None,
):
    # This copied directly from dataclasses as I figured it'd work here as well
    # Minor modifications have been made based on some assumptions I have made

    if locals is None:
        locals = {}

    locals.update({f"_{arg_name}_type": arg_type for (arg_name, arg_type) in args})
    locals["_return_type"] = return_type
    return_annotation = "-> _return_type"

    arg_txt = ", ".join([f"{arg_name}: _{arg_name}_type" for (arg_name, _) in args])
    body = "\n".join(f"  {b}" for b in body)

    # Compute the text of the entire function.
    txt = f" def {name}({arg_txt}){return_annotation}:\n{body}"

    local_vars = ", ".join(locals.keys())
    txt = f"def __create_fn__({local_vars}):\n{txt}\n return {name}"
    ns = {}
    exec(txt, globals, ns)
    return ns["__create_fn__"](**locals)


def _process_class(
    cls: type,
    generate_init: bool,
    endianness: endianness,
    alignment: int,
    packed: bool,
    serialize: bool,
    deserialize: bool,
):
    # This function is heavily based on how dataclasses' solve the issue of type introspection
    __ignored_attributes = {
        # Basic python attributes
        "__init__",
        "__slots__",
        "__args__",
        "__parameters__",
        "__module__",
        "__annotations__",
        "__orig_bases__",
        "__dict__",
        "__weakref__",
        "__doc__",
        "__parameters__",
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
        if not _supported_type(_type_spec["base"]):
            breakpoint()
            raise TypeError(
                f"Field {name!r} in {cls!r} is annotated with an unsupported type"
            )
        else:
            fields[name] = _determine_type(_type)

    setattr(cls, "_CFIELDS", fields)

    # TODO: Figure out which fields have initializers
    initialized: set[str] = set()

    # TODO: Generate getters and delete setters for constant fields

    if generate_init:
        init_fields = [
            (name, _type)
            for name, _type in annotations.items()
            if name not in initialized
        ]
        init_fn = _create_fn("__init__", [("self", cls)] + init_fields, cls)
        init_fn.__qualname__ = f"{cls.__qualname__}.{init_fn.__name__}"
        if "__init__" not in cls.__dict__:
            setattr(cls, "__init__", init_fn)

    return cls


def cstruct(
    cls: type | None = None,
    /,
    *,
    init: bool = True,
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
            cls, init, endianness, alignment, packed, serialize, deserialize
        )

    # Allows for use by both @cstruct and cstruct()
    return wrap if cls is None else wrap(cls)
