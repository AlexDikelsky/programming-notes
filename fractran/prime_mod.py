#!/usr/bin/env python3
import math

a = [17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 1, 55]
b = [91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11,  2, 7,  1]

N = 2
ns = []

def is_integral(N, a, b):
    return (N*a) % b == 0

runs = 0
while runs < 10000000:
    i = 0
    found = False
    while i < len(a) and not found:
        if is_integral(N, a[i], b[i]):
            #print(N)
            ns.append([N, runs])
            N = (N * a[i]) // b[i]
            found = True
        i += 1
    runs += 1

pows = [2**i for i in range(10)]
print(pows)
for val in ns:
    if val[0] in pows:
        print(val, int(math.log(val[0], 2)))
