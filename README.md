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
Or if you want `point_array` to have arbitrary sizes you can do this:
```py
from typing import Generic, TypeVarTuple
Shape = TypeVarTuple("Shape")

@cstruct
class point_array(Generic[*Shape]):
    points: vec2f[*shape]

```