
import re
import bisect


class RangeMapper:
    def __init__(self, dest_start: int, source_start: int, range_length: int) -> None:
        self.source_range = (source_start, source_start + range_length - 1)
        self.difference = dest_start - source_start

    def is_in_bounds(self, source: int) -> bool:
        return source >= self.source_range[0] and source <= self.source_range[1]

    def map_source_to_dest(self, source: int):
        return source + self.difference

    def __str__(self) -> str:
        dest_tuple = (
            self.source_range[0] + self.difference, self.source_range[1] + self.difference)
        return f'{self.source_range} -> {dest_tuple}'


class Mapper:
    def __init__(self):
        self.range_mappers: list[RangeMapper] = []

    def add_mapper(self, range_mapper: RangeMapper):
        bisect.insort(self.range_mappers, range_mapper,
                      key=lambda m: m.source_range[0])

    def map_source_to_dest(self, source: int):
        for rm in self.range_mappers:
            if rm.is_in_bounds(source):
                return rm.map_source_to_dest(source)
        return source

    @staticmethod
    def from_list(list_of_integers: list[int]) -> 'Mapper':
        assert len(list_of_integers) % 3 == 0
        instance = Mapper()
        for i in range(0, len(list_of_integers), 3):
            instance.add_mapper(RangeMapper(
                list_of_integers[i], list_of_integers[i + 1], list_of_integers[i + 2]))
        return instance


class SequentialMapper:
    def __init__(self):
        self.mappers: list[Mapper] = []

    def add_mapper(self, mapper: Mapper):
        self.mappers.append(mapper)

    def map_source_to_dest(self, source: int):
        current: int = source
        for mapper in self.mappers:
            current = mapper.map_source_to_dest(current)
        return current


if __name__ == '__main__':
    with open("input.txt", "r") as f:
        file_str: str = f.read()
        file_str = file_str.replace("\n", " ")
    splits = file_str.split(":")
    splits.pop(0)

    seeds = list(
        map(int, re.sub(r"[a-zA-Z-]", "", splits[0]).strip().split(" ")))
    full_mapper = SequentialMapper()
    for i in range(1, 8):
        full_mapper.add_mapper(Mapper.from_list(
            list(map(int, re.sub(r"[a-zA-Z-]", "", splits[i]).strip().split(" ")))))

    all_results = list(
        map(lambda seed: full_mapper.map_source_to_dest(seed), seeds))
    all_results.sort()
    print(all_results[0])
