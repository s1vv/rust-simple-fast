# type: ignore
import numpy as np


def main():
    print("Hello from py!")


symbols = np.array(["H", "W", "A", "O"])
weights = np.array([1.36, 1.00, 0.87, 0.8], dtype=np.float64)

symbol_to_index = {"H": 0, "W": 1, "A": 2, "O": 3}


def separate_liquids_numpy(glass):
    if not glass:
        return []

    glass_width = len(glass[0])

    # 1️⃣ flat → индексы
    flat_indices = np.fromiter(
        (symbol_to_index.get(c, -1) for row in glass for c in row), dtype=np.int8
    )

    # 2️⃣ Индексы → веса
    flat_weights = weights[flat_indices]

    # 3️⃣ Получаем сортировку
    sorted_indices = np.argsort(flat_weights)

    # 4️⃣ Применяем сортировку к исходным индексам
    sorted_flat_indices = flat_indices[sorted_indices]

    # 5️⃣ Превращаем обратно в символы
    sorted_symbols = symbols[sorted_flat_indices]

    # 6️⃣ reshape
    return sorted_symbols.reshape(-1, glass_width).tolist()


if __name__ == "__main__":
    main()
