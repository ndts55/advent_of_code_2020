from typing import Tuple
from math import ceil, floor
from functools import reduce

seats = [s.strip() for s in  open('05.txt').readlines()]


def seat_id(row: int, col: int) -> int:
    return row * 8 + col


Pair = Tuple[int, int]


def middle(a: int, b: int) -> float:
    return a/2 + b/2


def rf(acc: Tuple[Pair, Pair], c: chr) -> Tuple[Pair]:
    horizontal, vertical = acc
    lower, upper = horizontal
    left, right = vertical
    if c == 'F':
        return ((lower, floor(middle(upper, lower))), vertical)
    elif c == 'B':
        return ((ceil(middle(upper, lower)), upper), vertical)
    elif c == 'L':
        return (horizontal, (left, floor(middle(left, right))))
    elif c == 'R':
        return (horizontal, (ceil(middle(left, right)), right))
    else:
        return acc

def untuple(tup: Tuple[Pair, Pair]) -> Pair:
    return (tup[0][0], tup[1][0])

InitialRanges: Tuple[Pair, Pair] = ((0, 127), (0, 7))

seat_ids = [seat_id(*untuple(reduce(rf, seat, InitialRanges))) for seat in seats]

max_seat_id = max(seat_ids)

print('part one')
print(max_seat_id)

print('part two')
for n in range(min(seat_ids), max_seat_id + 1):
    if n not in seat_ids:
        print(n)
        break
