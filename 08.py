from typing import List, Tuple, Optional


def execute(acc: int, inst: int, operation: str, argument: int) -> Tuple[int, int]:
    if operation == 'acc':
        acc += argument
        inst += 1
    elif operation == 'nop':
        inst += 1
    elif operation == 'jmp':
        inst += argument

    return acc, inst


inst = 0
acc = 0
instructions = [(o, int(a))
                for o, a in (tuple(s.strip().split(' '))
                             for s in open('08.txt').readlines())]
execution_marker = {x: False for x in range(len(instructions))}

while not execution_marker[inst]:
    execution_marker[inst] = True
    acc, inst = execute(acc, inst, *instructions[inst])

print('part one')
print(acc)


def execute_instructions(instructions: List[Tuple[str, int]]) -> Tuple[int, bool]:
    inst = 0
    acc = 0
    execution_marker = {x: False for x in range(len(instructions))}
    while inst < len(instructions) and not execution_marker[inst]:
        execution_marker[inst] = True
        acc, inst = execute(acc, inst, *instructions[inst])

    return acc, inst >= len(instructions)


def flip(instructions: List[Tuple[str, int]], n: int) -> Optional[List[Tuple[str, int]]]:
    if n >= len(instructions) or instructions[n][0] == 'acc':
        return None

    instructions = instructions.copy()
    o, a = instructions[n]
    if o == 'jmp':
        o = 'nop'
    else:
        o = 'jmp'

    instructions[n] = o, a

    return instructions

acc = 0
for i in range(len(instructions)):
    new_instructions = flip(instructions, i)
    if new_instructions is not None:
        acc, success = execute_instructions(new_instructions)
        if success:
            break

print('part two')
print(acc)
