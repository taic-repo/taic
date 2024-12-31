import re
import sys
import os
import numpy as np


def parse(data):
    match = re.search(r'(.*?)\[(.*?)\]', data)
    if match:
        return match.groups()
    else:
        return None
    
def main():
    filename = sys.argv[1]
    dirname = os.path.dirname(filename)
    with open(filename, 'r') as file:
        lines = file.readlines()
        for line in lines:
            matches = parse(line)
            if matches:
                fname = os.path.basename(dirname) + "_" + matches[0].split(':')[0]
                fname = fname.replace(" ", "_")
                fname = fname.lower() + ".dat"
                arrays = matches[1].split(',')
                data = np.array([int(x) for x in arrays])

                Q1 = np.percentile(data, 25)
                Q3 = np.percentile(data, 75)

                # 计算四分位距
                IQR = Q3 - Q1

                # 定义异常值的界限
                lower_bound = Q1 - 1.5 * IQR
                upper_bound = Q3 + 1.5 * IQR

                # 过滤异常值
                filtered_data = data[(data >= lower_bound) & (data <= upper_bound)]
                print("Mean: ", np.mean(filtered_data))
                print("Standard Deviation: ", np.std(filtered_data))
                np.savetxt("assets/" + fname, filtered_data, fmt='%d')

if __name__ == "__main__":
    main()