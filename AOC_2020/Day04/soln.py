DEBUG = True


class DumbPassport:
    def __init__(self, *, byr, iyr, eyr, hgt, hcl, ecl, pid, cid=-1):
        self.byr = byr
        self.iyr = iyr
        self.eyr = eyr
        self.hgt = hgt
        self.hcl = hcl
        self.ecl = ecl
        self.pid = pid
        self.cid = cid


class Passport:
    def __init__(self, *, byr, iyr, eyr, hgt, hcl, ecl, pid, cid=-1):
        self.byr = int(byr)
        self.iyr = int(iyr)
        self.eyr = int(eyr)
        self.hgt = hgt
        self.hcl = hcl
        self.ecl = ecl
        self.pid = pid
        self.cid = cid

    def validate(self):
        valid = True
        valid &= 1920 <= self.byr <= 2002
        valid &= 2010 <= self.iyr <= 2020
        valid &= 2020 <= self.eyr <= 2030

        valid &= self.valid_hgt()
        valid &= self.valid_hcl()
        valid &= self.valid_ecl()
        valid &= (len(self.pid) == 9) & self.valid_pid()

        if not valid:
            raise Exception

    def valid_hgt(self):
        unit = self.hgt[-2:]
        valid = False
        try:
            val = int(self.hgt[:-2])
            if unit == "in":
                valid = 59 <= val <= 76
            elif unit == "cm":
                valid = 150 <= val <= 193
        finally:
            return valid

    def valid_hcl(self):
        valid = len(self.hcl) == 7
        valid &= self.hcl[0] == "#"

        valid_chars = set("0123456789abcdef")
        valid &= all([c in valid_chars for c in self.hcl[1:]])
        return valid

    def valid_ecl(self):
        options = set(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
        return self.ecl in options

    def valid_pid(self):
        valid = False
        try:
            int(self.pid)
            valid = True
        finally:
            return valid


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()
    return content


def parse_line_dumb(line):
    passport = None
    success = False
    try:
        parsed_line = (
            line.replace("\n", " ")
            .replace(":", '="')
            .replace("  ", " ")
            .replace(" ", '", ')
            + '"'
        )
        passport = eval(f"DumbPassport({parsed_line})")
        success = True
    finally:
        return success, passport


def parse_line(line):
    passport = None
    success = False
    try:
        parsed_line = (
            line.replace("\n", " ")
            .replace(":", '="')
            .replace("  ", " ")
            .replace(" ", '", ')
            + '"'
        )
        passport = eval(f"Passport({parsed_line})")
        passport.validate()
        success = True
    finally:
        return success, passport


def task_1():
    content = read_src()
    content = "\n".join(content)
    content = content.split("\n\n")
    valid = []
    for line in content:
        if line == "":
            continue
        success, passport = parse_line_dumb(line)
        if success:
            valid.append(passport)
    print(f"task 1: {len(valid)}")


def task_2():
    content = read_src()
    content = "\n".join(content)
    content = content.split("\n\n")
    valid = []
    for line in content:
        if line == "":
            continue
        success, passport = parse_line(line)
        if success:
            valid.append(passport)
    print(f"task 2: {len(valid)}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
