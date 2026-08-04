class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
       """create a hashmap for each invalidity condition 
       (same value in row, column or grid)"""
       rows = [set() for _ in range(9)]
       cols = [set() for _ in range(9)]
       grid = [set() for _ in range(9)]

       for r in range(9):
        for c in range(9):
            val = board[r][c]
            if val == '.':
                """if the value is empty then just move on
                to the next value"""
                continue
            grid_num = (r//3)*3 + (c//3)

            if val in rows[r] or val in cols[c] or val in grid[grid_num]:
                return False
            rows[r].add(val)
            cols[c].add(val)
            grid[grid_num].add(val)
       return True