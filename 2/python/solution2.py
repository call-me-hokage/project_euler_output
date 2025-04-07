limit = 400000

def fibonacci(limit):
    total = 0
    a,b = 1,1
    while True:
        c  = a+b
        if c > limit:
            break
        if c % 2 ==0:
            total += c
        a = b
        b = c 
    return total

total = fibonacci(limit)
print(total)
