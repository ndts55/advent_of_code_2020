from typing import List
from operator import sub
from functools import lru_cache

joltages = [int(s.strip()) for s in open('10.txt').readlines()]

device_joltage = max(joltages) + 3
output_joltage = 0

s_j = sorted(joltages)

diffs = list(map(sub, s_j + [device_joltage], [output_joltage] + s_j))

print('part one')
print(diffs.count(1) * diffs.count(3))


def count_combinations(joltages: List[int], device_joltage: int, output_joltage: int) -> int:
    @lru_cache(maxsize=None)
    def f_rec(joltage: int) -> int:
        # if we're already within range of device_joltage return 1
        if (device_joltage - joltage) <= 3:
            return 1
        return sum(f_rec(x) for x in joltages if 0 < (x - joltage) <= 3)
    
    return f_rec(output_joltage)

print('part two')
print(count_combinations(joltages, device_joltage, output_joltage))
