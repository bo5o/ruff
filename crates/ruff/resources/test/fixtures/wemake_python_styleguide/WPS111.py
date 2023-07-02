from matplotlib import pyplot as p  # bad
from matplotlib import pyplot as plt  # good

import numpy as n  # bad
import numpy as np  # good

x = 1  # bad
x_coordinate = 1  # good

y = 2  # bad
abscissa = 2  # good

z: int = 1  # bad
z_coordinate: int = 1  # good

c: str | bytes = "drop foo bar"  # bad
command: str | bytes = "drop foo bar"  # good

_ = "unused"  # good


def f(a: int, b):  # bad
    pass


def some_function(arg1: int, arg2):  # good
    pass


class C:  # bad
    pass


class SomeClass:  # good
    pass


match command.split():
    case ["quit" as c]:  # bad
        pass
    case ["go", d]:  # bad
        pass
    case ["drop", *o]:  # bad
        pass


match command.split():
    case ["quit" as cmd]:  # good
        pass
    case ["go", direction]:  # good
        pass
    case ["drop", *objects]:  # good
        pass
