import xlsxwriter
import os
from datetime import datetime

ifile = os.getenv('ifile', './sample.d/input.xlsx')
os.makedirs(os.path.dirname(ifile), exist_ok=True)

workbook = xlsxwriter.Workbook(ifile)

# Sheet1 with various types
ws1 = workbook.add_worksheet('Sheet1')
data1 = [
    ['StringValue', 42, 123.45],
    [True, None, 'AnotherString'],
    [1000000000000, 10, 'Target'],
]
for r, row in enumerate(data1):
    for c, val in enumerate(row):
        ws1.write(r, c, val)

# Sheet2 with large integers
ws2 = workbook.add_worksheet('SheetBigInt')
ws2.write(0, 0, 9223372036854775807) # Max i64
ws2.write(1, 0, 42)

# Sheet3 with booleans
ws3 = workbook.add_worksheet('SheetBool')
ws3.write(0, 0, False)
ws3.write(1, 0, True)

# Sheet4 with empty cells (explicitly surrounded to ensure they are in the used range)
ws4 = workbook.add_worksheet('SheetEmpty')
ws4.write(0, 0, 'Start')
ws4.write(1, 1, None) # Empty
ws4.write(2, 2, 'End')

workbook.close()
