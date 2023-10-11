nums = [3, 4, 5, 1, 2]
# Output: 1


def min_finder(lst: list) -> int:
    l = 0
    r = len(lst)
    mid = (l + r) // 2
    previous = mid - 1

    while True:
        mid = (l + r) // 2
        if lst[previous] > lst[mid]:
            return lst[mid]
        if lst[mid] > lst[r]:
            l = mid + 1
        else:
            r = mid - 1
