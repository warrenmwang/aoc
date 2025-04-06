def is_nice(s: str) -> bool:
    vowels = []
    twice_in_row = False

    for i, c in enumerate(s):
        # vowel counting
        if c == "a" or c == "e" or c == "i" or c == "o" or c == "u":
            vowels.append(c)

        if i != len(s) - 1:
            # twice in a row chars
            if not twice_in_row:
                if c == s[i+1]:
                    twice_in_row = True

            # contains bad sequence
            tmp = f"{c}{s[i+1]}"
            if tmp == "ab" or tmp == "cd" or tmp == "pq" or tmp == "xy":
                return False

    if len(vowels) >= 3 and twice_in_row:
        return True
    else:
        return False

def part_one():
    print(is_nice("ugknbfddgicrmopn") == True)
    print(is_nice("aaa") == True)
    print(is_nice("aa") == False) # only two vowels
    print(is_nice("jchzalrnumimnmhp") == False) # no double letter
    print(is_nice("haegwjzuvuyypxyu") == False) # contains xy
    print(is_nice("dvszwmarrgswjxmb") == False) # one vowel

    num_nice = 0
    with open("input/5.txt", "r") as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            if is_nice(line):
                num_nice += 1
    print(num_nice) # 255

def is_nice_too(s: str) -> bool:
    double_pair = False
    pairs = {}
    for i in range(len(s) - 1):
        pair = f"{s[i]}{s[i+1]}"
        if pair not in pairs:
            pairs[pair] = [i,i+1]
        elif i != pairs[pair][1]:
            # no overlap
            double_pair = True
            break

    triplet = False
    for i in range(len(s) - 2):
        if s[i] == s[i+2]:
            triplet = True
            break

    return double_pair and triplet


def part_two():
    print(is_nice_too("qjhvhtzxzqqjkmpb") == True)
    print(is_nice_too("xxyxx") == True)
    print(is_nice_too("uurcxstgmygtbstg") == False)
    print(is_nice_too("ieodomkazucvgmuy") == False)

    num_nice = 0
    with open("input/5.txt", "r") as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            if is_nice_too(line):
                num_nice += 1
    print(num_nice) # 55


if __name__ == "__main__":
    # part_one()
    part_two()
