import queue
from typing import List


def breadth_first_search(row_idx: int, col_idx: int, grid: List[List[int]]) -> int:
    cells_to_visit = queue.Queue()

    num_island_cells = 0
    cells_to_visit.put((row_idx, col_idx))

    while not cells_to_visit.empty():
        (r_idx, c_idx) = cells_to_visit.get()

        # left
        if c_idx >= 1 and grid[r_idx][c_idx - 1] == 1:
            cells_to_visit.put((r_idx, c_idx - 1))

        # right
        if c_idx >= 1 and grid[r_idx][c_idx + 1] == 1:
            cells_to_visit.put((r_idx, c_idx + 1))

        # up
        if c_idx >= 1 and grid[r_idx + 1][c_idx] == 1:
            cells_to_visit.put((r_idx + 1, c_idx))

        # down
        if c_idx >= 1 and grid[r_idx - 1][c_idx] == 1:
            cells_to_visit.put((r_idx - 1, c_idx))

        num_island_cells += 1
        grid[r_idx][c_idx] = 0
    return num_island_cells


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        max_area_so_far = 0
        for row_idx, row in enumerate(grid):
            for col_idx, col in enumerate(row):
                if row[col_idx] == 1:
                    area_of_island = breadth_first_search(row_idx, col_idx, grid)
                    max_area_so_far = max(area_of_island, max_area_so_far)
        return max_area_so_far


if __name__ == "__main__":
    berries = Solution()
    grid = [
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ]
    print(berries.maxAreaOfIsland(grid))
