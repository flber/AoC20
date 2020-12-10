import math
import re

with open('input.txt') as f:
    data = [x.strip() for x in f.readlines()]
    f.close()

count = 0
bags = []
parents = []
for x in data:
    tmp = x.replace('.','').replace(' bags','').replace(' bag','')
    tmp = re.sub(r'([0-9]){1} ', '', tmp)
    sp = tmp.split(" contain ")
    bag = sp[0]
    par = []
    for y in data:
        tmp2 = y.replace('.','').replace(' bags','').replace(' bag','')
        tmp2 = re.sub(r'([0-9]){1} ', '', tmp2)
        sp = tmp2.split(" contain ")
        if bag in sp[1]:
            par.append(sp[0])
    bags.append(bag)
    parents.append(par)

searched = []
toSearch = parents[bags.index('shiny gold')].copy()
count = len(parents[bags.index('shiny gold')])

while len(toSearch):
    index = bags.index(toSearch[0])
    par = parents[index]
    for x in par:
        if x not in searched and x not in toSearch:
            count += 1
            toSearch.append(x)
    searched.append(toSearch[0])
    toSearch.pop(0)
        
print(count)
