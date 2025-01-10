import re
import sys
import os
import numpy as np


def parse(data):
    match = re.search(r'([0-9].*?)msg/s', data)
    if match:
        return match.groups()
    else:
        return None
    
def main():
    filename = sys.argv[1]
    dirname = os.path.dirname(filename)
    fname = os.path.basename(filename)
    with open(filename, 'r') as file:
        content = file.read()
        res_1 = content.split('============ RESULTS ================')
        res_2 = []
        for res in res_1:
            matches = parse(res)
            if matches:
                res_2.append(matches[0])
        
        data = np.array([int(x) for x in res_2])

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
        np.savetxt("stat/" + fname, filtered_data, fmt='%d')

if __name__ == "__main__":
    main()