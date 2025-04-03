# part 1
# total = 0
# with open("2.input", "r") as f:
#     lines = f.readlines()
#     for l in lines:
#         l,w,h = map(int, l.split("x"))
#         s1 = l*w
#         s2 = w*h
#         s3 = h*l
#         total += 2*s1 + 2*s2 + 2*s3 + min(s1, s2, s3)
# print(total)

# part 2
total = 0
with open("2.input", "r") as f:
    lines = f.readlines()
    for l in lines:
        dims = [x for x in map(int, l.split("x"))]
        dims.sort()
        s1, s2, s3 = dims[0], dims[1], dims[2]
        total += 2*s1 + 2*s2 + (s1*s2*s3)

print(total)

