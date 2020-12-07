from functools import reduce
from operator import and_

file_content = [s.strip() for s in open('06.txt').read().split('\n\n')]

answers1 = [s.replace('\n', '') for s in file_content]

counts1 = [len(set(a)) for a in answers1]

print('part one')
print(sum(counts1))

answers2 = [s.split('\n') for s in file_content]

counts2 = [len(reduce(and_, [set(a) for a in answer])) for answer in answers2]

print('part two')
print(sum(counts2))
