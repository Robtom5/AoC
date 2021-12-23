DEBUG = True

import re

scanner_regex = re.compile(r"--- scanner (?P<scan_no>\d+) ---")
beacon_regex = re.compile(r"(?P<dx>-?\d+),(?P<dy>-?\d+),(?P<dz>-?\d+)")


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    scanners = {}
    active_scanner = None
    for line in content:
        scanner = scanner_regex.match(line)
        if scanner is not None:
            scanner_id = int(scanner.group("scan_no"))
            scanners[scanner_id] = []
            active_scanner = scanners[scanner_id]
        else:
            beacon = beacon_regex.match(line)
            if beacon is not None:
                dx = int(beacon.group("dx"))
                dy = int(beacon.group("dy"))
                dz = int(beacon.group("dz"))
                active_scanner.append((dx, dy, dz))
            else:
                active_scanner = None

    return scanners


def task_1():
    scanners = read_src()
    # print(scanners)

    # Use vectors for each beacon to beacon?
    print(f"task 1: ")


def task_2():
    content = read_src()
    print(f"task 2: ")


if __name__ == "__main__":
    # DEBUG = False
    task_1()
    task_2()
