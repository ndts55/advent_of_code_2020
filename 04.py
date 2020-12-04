pps = [pp.replace('\n', ' ').split(' ')
       for pp in open('04.txt').read().split('\n\n')]

required_entries = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

print("part one")
print(
    sum(
        all(
            any(ppe.startswith(e) for ppe in pp)
            for e in required_entries)
        for pp in pps))


def hgt_validation(s):
    if s.endswith('cm'):
        return 150 <= int(s.split('cm')[0]) <= 193
    elif s.endswith('in'):
        return 59 <= int(s.split('in')[0]) <= 76
    else:
        return False


def hcl_validation(s):
    if not s.startswith('#') or len(s) != 7:
        return False
    return all(
        c.isdigit() or ord(c) in range(ord('a'), ord('f')+1)
        for c in s.split('#')[1])


entries_and_validations = {
    'byr': lambda s: 1920 <= int(s) <= 2002,
    'iyr': lambda s: 2010 <= int(s) <= 2020,
    'eyr': lambda s: 2020 <= int(s) <= 2030,
    'hgt': hgt_validation,
    'hcl': hcl_validation,
    'ecl': lambda s: s in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'],
    'pid': lambda s: len(s) == 9 and s.isdigit(),
}

print("part two")

print(
    sum(
        all(
            any(
                ppe.startswith(k) and v(ppe.split(':')[1])
                for ppe in pp
            )
            for k, v in entries_and_validations.items()
        )
        for pp in pps
    )
)
