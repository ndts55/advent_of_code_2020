from typing import List, Set
import re
from functools import reduce

rules = [s.strip() for s in open('07.txt').readlines()]

my_color = "shiny gold"


def transitive(rs: List[str], start: str) -> Set[str]:
    colors = {start}
    done = False
    while not done:
        old_len = len(colors)
        colors |= {' '.join(r.split()[:2]) for r in rs if any(
            c in r and not r.startswith(c) for c in colors)}
        done = old_len == len(colors)

    return colors ^ {start}


print('part one')
print(len(transitive(rules, my_color)))

reg = re.compile(r'(?P<number>\d+) (?P<color>\w+\s\w+)')


def internal_bag_count(rules: List[str], color: str) -> int:
    def rec(color: str) -> int:
        rule = [r for r in rules if r.startswith(color)][0]
        return reduce(
            lambda acc, match: acc +
            int(match.group('number')) *
            max(rec(match.group('color')), 1),
            reg.finditer(rule),
            1)
    return rec(color) - 1


print('part two')
print(internal_bag_count(rules, my_color))
