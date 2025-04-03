def calculate_visits_1(directions: str) -> int:
    pos = [0,0]
    d = {
        "[0, 0]": True
    }

    for c in directions:
        if c == "^":
            pos[1] += 1
        elif c == ">":
            pos[0] += 1
        elif c == "v":
            pos[1] -= 1
        elif c == "<":
            pos[0] -= 1
        elif c == "\n":
            break
        else:
            print("Unexpected value")
            exit(1)

        new_pos = f"{pos}"
        if new_pos not in d:
            d[new_pos] = True

    return len(d)

def calculate_visits_2(directions: str) -> int:
    pos1 = [0,0]
    pos2 = [0,0]
    d = {
        "[0, 0]": True
    }
    for i, c in enumerate(directions):
        if i % 2 == 0:
            if c == "^":
                pos1[1] += 1
            elif c == ">":
                pos1[0] += 1
            elif c == "v":
                pos1[1] -= 1
            elif c == "<":
                pos1[0] -= 1
            elif c == "\n":
                break
            else:
                print("Unexpected value")
                exit(1)
        else:
            if c == "^":
                pos2[1] += 1
            elif c == ">":
                pos2[0] += 1
            elif c == "v":
                pos2[1] -= 1
            elif c == "<":
                pos2[0] -= 1
            elif c == "\n":
                break
            else:
                print("Unexpected value")
                exit(1)

        new_pos1 = f"{pos1}"
        new_pos2 = f"{pos2}"
        if new_pos1 not in d:
            d[new_pos1] = True
        if new_pos2 not in d:
            d[new_pos2] = True


    return len(d)

if __name__ == "__main__":

    # print(calculate_visits_1(">")) # 2
    # print(calculate_visits_1("^>v<")) # 4
    # print(calculate_visits_1("^v^v^v^v^v")) # 2
    # with open("./3.input", "r") as f:
    #     line = f.readline()
    #     print(calculate_visits_1(line)) # 2572

    with open("./3.input", "r") as f:
        line = f.readline()
        print(calculate_visits_2(line)) # 2631

        # print(calculate_visits_2("^>") == 3)
        # print(calculate_visits_2("^>v<") == 3)
        # print(calculate_visits_2("^v^v^v^v^v") == 11)
