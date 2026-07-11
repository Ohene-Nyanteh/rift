# Python equivalent of the Rift benchmark
import time

class Data:
    def __init__(self):
        self.id = 0
        self.a = self.b = self.c = self.d = self.e = 0
        self.f = self.g = self.h = self.i = self.j = 0
        self.k = self.l = self.m = self.n = self.o = 0
        self.p = self.q = self.r = self.s = self.t = 0
        self.u = "hello"
        self.v = "world"
        self.w = "rift"
        self.x = "test"
        self.y = "data"
        self.items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                      11, 12, 13, 14, 15, 16, 17, 18, 19, 20]

def level5(d):
    return d.id
def level4(d):
    return level5(d)
def level3(d):
    return level4(d)
def level2(d):
    return level3(d)
def level1(d):
    return level2(d)

base = Data()
base.id = 42

start = time.time()
for i in range(1_000_000):
    result = level1(base)
end = time.time()

print(f"done ({end - start:.2f}s)")
