with open('input copy.txt') as file:
    sum = 0
    for line in file:
        sum += int(line.strip())
    print(sum)