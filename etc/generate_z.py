#   Generates all arithmetic expressions of the form a + b = c and a - b = c with abs(a), abs(b), abs(c) <= N.
#
#   Invoke as: python generate_z.py N

from collections import defaultdict
from functools import total_ordering

ADD = 'add'
NEG = 'neg'
SUB = 'sub'

MUL = 'mul'
DIV = 'div'
REM = 'rem'

STEPS = { ADD: 3, NEG: 3, SUB: 3, MUL: 3, DIV: 3, REM: 3 }


class Bucket:
    def __init__(self, name, low, high):
        self.name = name
        self.low = low
        self.high = high
        self.operations = []


@total_ordering
class BinaryOperation:
    def __init__(self, name, a, op, b, c):
        self.name = name
        self.a = a
        self.b = b
        self.op = op
        self.c = c

    def magnitude(self):
        return max(abs(self.a), abs(self.b), abs(self.c))

    def print(self):
        return f"impl_z!({self.a} {self.op} {self.b} = {self.c});"

    def __lt__(self, other):
        return (self.a, self.b, self.op, self.c) < (other.a, other.b, other.op, other.c)


@total_ordering
class UnaryOperation:
    def __init__(self, name, op, x, c):
        self.name = name
        self.x = x
        self.op = op
        self.c = c

    def magnitude(self):
        return max(abs(self.x), abs(self.c))

    def print(self):
        return f'impl_z!({self.op} {self.x} = {self.c});'

    def __lt__(self, other):
        return (self.x, self.op, self.c) < (other.x, other.op, other.c)


def generate_buckets(n):
    """Generates the buckets to put the generated operations into"""

    def gen_into(buckets, name, n):
        step = STEPS[name]

        n = (n + step - 1) // step * step

        for i in range(1, n+1, step):
            low, high = i, i + step - 1

            buckets[(name, low)] = Bucket(name, low, high)

    buckets = dict()

    gen_into(buckets, ADD, n)
    gen_into(buckets, NEG, n)
    gen_into(buckets, SUB, n)

    gen_into(buckets, MUL, n)
    gen_into(buckets, DIV, n)
    gen_into(buckets, REM, n)

    return buckets


def generate_all(n):
    """Generates all expressions of the form {op} a = c and a {op} b = c for a, b and c in [-n, n]"""

    for a in range(-n, n+1):
        c = -a

        if abs(c) <= n:
            yield UnaryOperation(NEG, 'neg', a, c)

        for b in range(-n, n+1):
            c = a + b

            if abs(c) <= n:
                yield BinaryOperation(ADD, a, '+', b, c)

            c = a - b

            if abs(c) <= n:
                yield BinaryOperation(SUB, a, '-', b, c)

            c = a * b

            if abs(c) <= n:
                yield BinaryOperation(MUL, a, '*', b, c)

            if b == 0:
                continue

            c = a // b

            if a == b * c and abs(c) <= n:
                yield BinaryOperation(DIV, a, '/', b, c)

            if abs(b) <= 1:
                continue

            c = a % b

            if abs(c) <= n:
                yield BinaryOperation(REM, a, '%', b, c)


def sort_into(operations, buckets):
    """Sorts a sequence of operations into multiple lists, by bucket"""

    for op in operations:
        step = STEPS[op.name]

        magnitude = op.magnitude()

        #   0 is handled unconditionally
        if magnitude == 0:
            continue

        magnitude = (magnitude - 1) // step * step + 1

        try:
            buckets[(op.name, magnitude)].operations.append(op)
        except KeyError as e:
            print(f'{op.magnitude()} and {step} -> {magnitude}')
            raise e


def print_all(buckets):
    """Prints it all"""

    def format_features(buckets):
        lines = []

        for b in buckets:
            step = b.high - b.low + 1

            feature = f'{b.name}-{b.low}-{b.high}'

            deps = f'["{b.name}-{b.low - step}-{b.high - step}"]' if b.low > step else '[]'

            lines.append(f'{feature} = {deps}')

        return ['[features]', '', 'default = ["add-4-6", "neg-4-6", "sub-4-6"]', ''] + lines

    def format_modules(buckets):
        lines = []

        for b in buckets:
            lines.append(f'#[cfg(feature = "{b.name}-{b.low}-{b.high}")]')
            lines.append(f'mod {b.name}_{b.low}_{b.high};')
            lines.append('')

        return lines

    def format_operations(operations):
        operations.sort()

        lines = [f'//! Implementation of {b.name} for numbers in {b.low}..={b.high}.', '']

        return lines + [o.print() for o in operations]

    def print_to(path, lines):
        with open(path, 'w') as file:
            for line in lines:
                file.write(line + '\n')

    sorted_buckets = sorted(buckets.values(), key = lambda b: (b.name, b.low))

    print_to('generated/features.toml', format_features(sorted_buckets))

    print_to('generated/z.rs', format_modules(sorted_buckets))

    for b in sorted_buckets:
        print_to(f'src/z/{b.name}_{b.low}_{b.high}.rs', format_operations(b.operations))


#
#   Script
#

if __name__ == "__main__":
    import sys

    n = int(sys.argv[1])

    buckets = generate_buckets(n)

    operations = generate_all(n)

    sort_into(operations, buckets)

    print_all(buckets)
