def div_ceil(n, d):
    return ceil(n / d )

def minEatingSpeed(self, piles, h):
    start = 0
    for pile in piles:
        start += pile
    start = div_ceil(start, h)
    end = max(piles)
    while start <= end:
        k = (start + end) // 2
        t = 0
        for pile in piles:
            t += div_ceil(pile, k)
        if t > h:
            start = k + 1
        else:
            end = k - 1


    return int(k)
    
