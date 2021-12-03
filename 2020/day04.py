with open("./Day04Input.txt") as infile:
    contents = infile.read()

passports = contents.split('\n\n')
passports = [p.replace('\n', ' ') for p in passports]

EXPECTED_FIELDS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
OPTIONAL_FIELDS = ['cid']

def parse_passport(passport):
    property_strings = passport.split(' ')
    properties = {}
    for key, value in [prop_string.split(':') for prop_string in property_strings if prop_string]:
        properties[key] = value
    return properties

def validate_passport(passport):
    valid = True
    for expected in EXPECTED_FIELDS:
        valid &= expected in passport
    return valid

valid_passports = [pp for pp in passports
                   if validate_passport(parse_passport(pp))]
print(f"Valid Passports: {len(valid_passports)}")

# Part 2

import re

BYR_MIN = 1920
BYR_MAX = 2002
IYR_MIN = 2010
IYR_MAX = 2020
EYR_MIN = 2020
EYR_MAX = 2030
HGT_REGEX = re.compile(r'^(?P<val>\d+)(?P<unit>in|cm)$')
HGT_IN_MIN = 59
HGT_IN_MAX = 76
HGT_CM_MIN = 150
HGT_CM_MAX = 193
HCL_REGEX = re.compile(r'^#([a-f\d]{6})$')
ECL_VALID = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
PID_REGEX = re.compile(r'^\d{9}$')

def validate_fields(passport):
    pass_dict = parse_passport(passport)

    byr = BYR_MIN <= int(pass_dict['byr']) <= BYR_MAX
    iyr = IYR_MIN <= int(pass_dict['iyr']) <= IYR_MAX
    eyr = EYR_MIN <= int(pass_dict['eyr']) <= EYR_MAX
    HGT_RAW = HGT_REGEX.match(pass_dict['hgt'])
    if HGT_RAW:
        if HGT_RAW['unit'] == 'cm':
            hgt = HGT_CM_MIN <= int(HGT_RAW['val']) <= HGT_CM_MAX
        else:
            hgt = HGT_IN_MIN <= int(HGT_RAW['val']) <= HGT_IN_MAX
    else:
        hgt = False
    hcl = HCL_REGEX.match(pass_dict['hcl'])
    ecl = pass_dict['ecl'] in ECL_VALID
    pid = PID_REGEX.match(pass_dict['pid'])

    return byr and iyr and eyr and hgt and hcl and ecl and pid

validated_passports = [vp for vp in valid_passports if validate_fields(vp)]
print(f"Validated Passports {len(validated_passports)}")
