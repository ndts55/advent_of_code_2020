from typing import List, Set, Tuple

numbers = [int(s.strip()) for s in open('09.txt').readlines()]


def sums25(ns: List[int]) -> Set[int]:
    return {i + j for i in ns for j in ns if i != j}


def misfit(ns: List[int]) -> int:
    for i in range(25, len(ns), 1):
        if ns[i] not in sums25(ns[i-25:i]):
            return ns[i]


misfit_n = misfit(numbers)

print('part one')
print(misfit_n)

def vr(ns: List[int], target: int) -> List[int]:
    for start in range(len(ns)):
        for end in range(start + 2, len(ns) + 1, 1):
            s = sum(ns[start:end])
            if s > target:
                break
            if s == target:
                return ns[start: end]

def minmaxsum(ns: List[int]) -> int:
    return max(ns) + min(ns)


print('part two')
print(minmaxsum(vr(numbers, misfit_n)))
