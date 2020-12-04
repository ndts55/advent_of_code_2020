from typing import List, Generator, Tuple
from functools import partial, reduce
from operator import mul

trees: List[str] = [line.strip() for line in open('12_03.txt').readlines()]

height = len(trees)
width = len(trees[0])

# generate wrapping indices


def index_generator(
        height: int,
        width: int,
        right: int,
        down: int) -> Generator[Tuple[int, int], None, None]:
    x = y = 0
    while y < height:
        yield (x, y)
        x = (x + right) % width
        y += down


tree_indices = partial(index_generator, height, width)


# map indices to tree and count
print("part one")
indices1 = tree_indices(3, 1)
part_one_result = sum(trees[row][col] == '#' for col, row in indices1)
print(part_one_result)

print("part two")
additional_slopes = [(1, 1), (5, 1), (7, 1), (1, 2)]

results = (
    sum(trees[row][col] == '#'
        for col, row in indices)
    for indices in (
        tree_indices(*slope)
        for slope in additional_slopes))

part_two_result = reduce(mul, results, 1) * part_one_result

print(part_two_result)
