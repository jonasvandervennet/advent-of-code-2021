from collections import defaultdict
from itertools import combinations_with_replacement


# PART 1: 428940

def main():
    with open("inputs/infi.txt") as ifp:
        lines = ifp.readlines()
    # lines = [
    #     "46 onderdelen missen",
    #     "Zoink: 9 Oink, 5 Dink",
    #     "Floep: 2 Flap, 4 Dink",
    #     "Flap: 4 Oink, 3 Dink"
    # ]
    missing_parts = int(lines[0].split(" onderdelen missen")[0])

    num_parts_per_key = defaultdict(lambda: 1)
    rules = defaultdict(lambda: [])
    for rule in lines[1:]:
        key, parts = rule.strip().split(": ")
        for consituent in parts.split(", "):
            amount, material = consituent.split(" ")
            rules[key].append((int(amount), material))

    # part 1
    done = False
    while not done:
        done = True
        for key, parts in rules.items():
            if key in num_parts_per_key.keys():
                continue
            if all([part[1] in num_parts_per_key.keys() or part[1] not in rules.keys() for part in parts]):
                num_parts_per_key[key] = sum([part[0]*num_parts_per_key[part[1]] for part in parts])
                done = False

    print(f"part 1: {max(num_parts_per_key.values())}")

    # part 2
    materials: list[str] = [material_amount[1] for material_amounts in rules.values() for material_amount in material_amounts]
    toys = [key for key in rules.keys() if key not in materials]

    NUM_PRESENTS = 20

    match_found = False
    for comb in combinations_with_replacement(toys, NUM_PRESENTS):
        parts = sum([num_parts_per_key[toy] for toy in comb])
        if parts == missing_parts:
            code = ''.join(sorted([name[0] for name in comb]))
            print(f"part 2: {code}")
            match_found = True
            break

    if not match_found:
        print("ERROR: Could not find correct amount of used parts")


if __name__ == "__main__":
    main()
