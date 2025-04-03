import hashlib

'''
# answer: 117946
# answer: 3938038
'''

def solve(n: int):
    expected = "0" * n
    key = "ckczppom"
    num = 1

    while True:
        input_ = f"{key}{num}"
        test = hashlib.md5(input_.encode()).hexdigest()
        if test[0:n] == expected:
            print(f"Found num {num} with key {key} yielding hash in hex format with {n} leading zeros: {test}")
            break
        else:
            num += 1

if __name__ == "__main__":
    solve(5)
    solve(6)
