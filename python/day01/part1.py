import os.path


def load_input():
    filename = os.path.join(
        os.path.dirname(__file__), "..", "..", "inputs", "day01.txt"
    )
    with open(filename) as input_fd:
        return [int(l.strip()) for l in input_fd.readlines()]


def get_fuel_for_mass(mass):
    return mass // 3 - 2


def main():
    print(sum([get_fuel_for_mass(x) for x in load_input()]))


if __name__ == "__main__":
    main()
