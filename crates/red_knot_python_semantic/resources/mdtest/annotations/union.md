# Union

## Annotation

`typing.Union` can be used to construct union types same as `|` operator.

```py
from typing import Union

a: Union[int, str]
a1: Union[int, bool]
a2: Union[int, Union[float, str]]
a3: Union[int, None]

def f():
    # revealed: int | str
    reveal_type(a)
    # Since bool is a subtype of int we merge both types to int here. But we do allow assigning boolean value
    # TODO: Pyright and Mypy do show this as `int | bool`: https://pyright-play.net/?code=GYJw9gtgBALgngBwJYDsDmUkQWEMoCqKSYKAUGQIYBchxpA2qjADRQBGYYANgLoUATAKbAowABQBKamShyoIIQDchlbgH14CIeMqSgA
    # Also mypy: https://mypy-play.net/?mypy=latest&python=3.12&gist=fd3c1609080ddc000d5247552c15b750
    # revealed: int
    reveal_type(a1)
    # revealed: int | float | str
    reveal_type(a2)
    # revealed: int | None
    reveal_type(a3)
```

## Assignment

```py
from typing import Union

a: Union[int, str]
a = 1
a = ""
a1: Union[int, bool]
a1 = 1
a1 = True
a2: int
# error: [invalid-assignment] "Object of type `Literal[""]` is not assignable to `int`"
a2 = a
```

## Typing Extensions

```py
from typing_extensions import Union

a: Union[int, str]

def f():
    # revealed: int | str
    reveal_type(a)
```