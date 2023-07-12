# Cerialzie

A declarative (de)serialization library with a dataclasses like interface.
In its simplest form it can look like this
```py
from cerialize import cstruct, f32

@cstruct
class vec2f:
    x: f32
    y: f32
```
This can also be declared as such:
```py
@cstruct
class vec2f:
    data: f32[2]
```
You can then reuse the class in another `cstruct`:
```py
@cstruct
class point_array:
    points: vec2f[10]
```

If you need your declaration to also support dimensions do this:
```py
from cerialize import cstruct, f64
from typing import TypeTupleVar, Generic
Shape = TypeTupleVar("Shape")

@cstruct
class complexf64(Generic[*Shape]):
    data: f64[2]

@cstruct
class complex_vec:
    data: complexf64[128, 512]
```
This means that `complex_vec` contains a field `data` which has 128 elements. Each of these elements have 512 subelements of type `complexf64`.