def update_grid(grid: list[list[int]], line: str) -> None:
    tmp = line.split(" ")
    end = tmp[-1]
    start = tmp[-3]

    action = ""
    if tmp[0] == "turn": action = tmp[1]
    else: action = tmp[0]

    start_x, start_y = list(map(int, start.split(",")))
    end_x, end_y = list(map(int, end.split(",")))

    # print(f"{action} {start_x},{start_y} -> {end_x},{end_y}")
    if action == "on":
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = 1
    elif action == "off":
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = 0
    else: # toggle
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = 1 if grid[row][col] == 0 else 0


def update_grid_part_two(grid: list[list[int]], line: str) -> None:
    tmp = line.split(" ")
    end = tmp[-1]
    start = tmp[-3]

    action = ""
    if tmp[0] == "turn": action = tmp[1]
    else: action = tmp[0]

    start_x, start_y = list(map(int, start.split(",")))
    end_x, end_y = list(map(int, end.split(",")))

    # print(f"{action} {start_x},{start_y} -> {end_x},{end_y}")
    if action == "on":
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = grid[row][col] + 1
    elif action == "off":
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = max(0, grid[row][col] - 1)
    else: # toggle
        for row in range(start_x, end_x + 1):
            for col in range(start_y, end_y + 1):
                grid[row][col] = grid[row][col] + 2



if __name__ == "__main__":
    # 0 = off, 1 = on

    def part_one():

        # grid = [[0 for _ in range(1000)] for _ in range(1000)]
        # update_grid(grid, "turn on 0,0 through 999,999")
        # print(sum(sum(l) for l in grid) == 1000000)
        #
        # grid = [[0 for _ in range(1000)] for _ in range(1000)]
        # update_grid(grid, "toggle 0,0 through 999,0")
        # print(sum(sum(l) for l in grid) == 1000)
        #
        # grid = [[0 for _ in range(1000)] for _ in range(1000)]
        # update_grid(grid, "turn on 499,499 through 500,500")
        # print(sum(sum(l) for l in grid) == 4)
        # update_grid(grid, "turn off 499,499 through 500,500")
        # print(sum(sum(l) for l in grid) == 0)

        grid = [[0 for _ in range(1000)] for _ in range(1000)]
        with open("./6.input", "r") as f:
            lines = f.readlines()
            for line in lines:
                update_grid(grid, line.strip())
        print(sum(sum(l) for l in grid))

    # part_one() # 569999

    def part_two():
        # grid = [[0 for _ in range(1000)] for _ in range(1000)]
        # update_grid_part_two(grid, "turn on 0,0 through 0,0")
        # print(sum(sum(l) for l in grid) == 1)
        # update_grid_part_two(grid, "turn off 0,0 through 0,0")
        # print(sum(sum(l) for l in grid) == 0)
        # update_grid_part_two(grid, "turn off 0,0 through 0,0")
        # print(sum(sum(l) for l in grid) == 0)
        #
        # grid = [[0 for _ in range(1000)] for _ in range(1000)]
        # update_grid_part_two(grid, "toggle 0,0 through 999,999")
        # print(sum(sum(l) for l in grid) == 2000000)

        grid = [[0 for _ in range(1000)] for _ in range(1000)]
        with open("./6.input", "r") as f:
            lines = f.readlines()
            for line in lines:
                update_grid_part_two(grid, line.strip())
        print(sum(sum(l) for l in grid))

    part_two() # 17836115

