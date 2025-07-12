# type: ignore
from separation.liquids_separation import (
    separate_liquids_numpy,
    separate_liquids_native,
)
import numpy as np


def generate_big_glass(n_rows=100_000, width=10):
    symbols = np.array(["H", "W", "A", "O"])
    # numpy random choice возвращает массив shape=(n_rows, width)
    glass = np.random.choice(symbols, size=(n_rows, width))
    # Преобразуем в list[list[str]] для совместимости с функцией
    return glass.tolist()


def test_benchmark_large_glass(benchmark):
    glass = generate_big_glass(100_000, 10)
    benchmark(lambda: separate_liquids_numpy(glass))


def test_benchmark_large_glass_native(benchmark):
    glass = generate_big_glass(100_000, 10)
    benchmark(lambda: separate_liquids_native(glass))


def test_simple_case():
    glass = [
        ["H", "H", "W", "O"],
        ["W", "W", "O", "W"],
        ["H", "H", "O", "O"],
    ]
    expected = [
        ["O", "O", "O", "O"],
        ["W", "W", "W", "W"],
        ["H", "H", "H", "H"],
    ]
    assert separate_liquids_numpy(glass) == expected


def test_empty_glass():
    glass = []
    expected = []
    assert separate_liquids_numpy(glass) == expected


def test_one_layer():
    glass = [["H", "W", "A", "O"]]
    expected = [["O", "A", "W", "H"]]  # По возрастанию веса
    assert separate_liquids_numpy(glass) == expected


def test_final():
    glass = [
        ["A", "O", "W", "W", "W", "A", "O"],
        ["H", "O", "H", "H", "A", "A", "W"],
        ["A", "A", "W", "O", "H", "A", "W"],
        ["H", "H", "A", "W", "O", "W", "H"],
        ["H", "O", "H", "A", "A", "O", "O"],
        ["W", "W", "H", "W", "A", "A", "A"],
    ]
    excepted = [
        ["O", "O", "O", "O", "O", "O", "O"],
        ["O", "A", "A", "A", "A", "A", "A"],
        ["A", "A", "A", "A", "A", "A", "A"],
        ["W", "W", "W", "W", "W", "W", "W"],
        ["W", "W", "W", "W", "H", "H", "H"],
        ["H", "H", "H", "H", "H", "H", "H"],
    ]
    assert separate_liquids_numpy(glass) == excepted
