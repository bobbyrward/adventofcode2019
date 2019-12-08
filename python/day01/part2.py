import os.path


def load_input():
    filename = os.path.join(
        os.path.dirname(__file__), "..", "..", "inputs", "day01.txt"
    )
    with open(filename) as input_fd:
        return [int(l.strip()) for l in input_fd.readlines()]


def get_fuel_for_mass(mass):
    return mass // 3 - 2


def get_fuel_for_module(module_mass):
    total_required = get_fuel_for_mass(module_mass)
    unaccounted_for = total_required

    while True:
        fuel_for_fuel = get_fuel_for_mass(unaccounted_for)
        if fuel_for_fuel <= 0:
            break

        total_required += fuel_for_fuel
        unaccounted_for = fuel_for_fuel

    return total_required


def main():
    print(sum([get_fuel_for_module(x) for x in load_input()]))


if __name__ == "__main__":
    main()
