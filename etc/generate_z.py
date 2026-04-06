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

POW = 'pow'
ROOT = 'root'

STEPS = { ADD: 3, NEG: 3, SUB: 3, MUL: 3, DIV: 3, REM: 3, POW: 3, ROOT: 3 }


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
        return f"impl_z!({self.a}i32 {self.op} {self.b}i32 = {self.c}i32);"

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
        return f'impl_z!({self.op} {self.x}i32 = {self.c}i32);'

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

    gen_into(buckets, POW, n)
    gen_into(buckets, ROOT, n)

    return buckets


def generate_operations(n):
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

            if b > 0:
                c = a // b

                if a == b * c and abs(c) <= n:
                    yield BinaryOperation(DIV, a, '/', b, c)

            if abs(b) > 1:
                c = a % b

                if abs(c) <= n:
                    yield BinaryOperation(REM, a, '%', b, c)

            if b >= 0:
                c = a ** b

                if abs(c) <= n:
                    yield BinaryOperation(POW, a, 'pow', b, c)

                #   The 0-th root of a number is not defined.
                #   There are two distinct n-th root of a number when n is even, causing conflicts.
                if abs(c) <= n and b > 0 and (a > 0 or b % 2 == 1):
                    yield BinaryOperation(ROOT, c, 'root', b, a)


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
            feature = f'{b.name}-{b.high}'

            deps = f'["{b.name}-{b.low - 1}"]' if b.low > 1 else '[]'

            lines.append(f'{feature} = {deps}')

        return ['[features]', '', 'default = ["add-6", "neg-6", "sub-6"]', ''] + lines

    def format_modules(buckets):
        lines = []

        for b in buckets:
            lines.append(f'#[cfg(feature = "{b.name}-{b.high}")]')
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

    operations = generate_operations(n)

    sort_into(operations, buckets)

    print_all(buckets)
