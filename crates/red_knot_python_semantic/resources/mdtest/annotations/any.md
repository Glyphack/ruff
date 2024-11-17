# Any

## Annotation

```py
from typing import Any

# reveal_type(Any)

b = object()
reveal_type(b)

a: Any

def f():
    # revealed: Any
    reveal_type(a)
```
