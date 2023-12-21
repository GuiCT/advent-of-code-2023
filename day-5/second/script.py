
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


class SeedRange:
    def __init__(self, start: int, end: int) -> None:
        self.start = start
        self.end = end

    def _is_contiguous(self, mapper: SequentialMapper) -> bool:
        start_map = mapper.map_source_to_dest(self.start)
        end_map = mapper.map_source_to_dest(self.end)
        diff_seed = self.end - self.start
        diff_map = end_map - start_map
        return diff_seed == diff_map

    def find_breaking_point(self, mapper: SequentialMapper) -> int | None:
        if self._is_contiguous(mapper):
            return None
        start_stack = [self.start]
        end_stack = [self.end]
        continue_loop = True
        while continue_loop:
            start_seed = start_stack[-1]
            end_seed = end_stack[-1]
            test_range = SeedRange(start_seed, end_seed)
            is_test_range_contiguous = test_range._is_contiguous(mapper)
            middle_seed = (start_seed + end_seed) // 2
            if is_test_range_contiguous:
                if end_stack[-1] == start_stack[-1]:
                    continue_loop = False
                start_stack.append(end_seed)
                end_stack.pop()
            else:
                end_stack.append(middle_seed)
        return start_stack[-1]


if __name__ == '__main__':
    with open("input.txt", "r") as f:
        file_str: str = f.read()
        file_str = file_str.replace("\n", " ")
    splits = file_str.split(":")
    splits.pop(0)

    seeds_range = list(
        map(int, re.sub(r"[a-zA-Z-]", "", splits[0]).strip().split(" ")))
    full_mapper = SequentialMapper()
    for i in range(1, 8):
        full_mapper.add_mapper(Mapper.from_list(
            list(map(int, re.sub(r"[a-zA-Z-]", "", splits[i]).strip().split(" ")))))

    range_list: list[SeedRange] = []
    for i in range(0, len(seeds_range), 2):
        range_list.append(
            SeedRange(seeds_range[i], seeds_range[i] + seeds_range[i + 1]))

    # Instead of trying every value in the input ranges (these can be VERY big, some
    # people got 100M values, some got 2B, 3B, ...) which would be unfeasible, we are
    # gonna take advantage of the characteristics of the linear mappings.

    # Every mapping we have is MONOTONE, that is: they only increase from a base value.

    # Then, for every range we have parsed, we check if they use the same mappings

    # That is: if x0 is the start and xn is the end, given a mapping y0, using the
    # same mappings, we expect yn = y0 + (xn - x1). When that happens, we call the
    # range CONTIGUOUS.

    # When that does not happen, we find the "breaking point" (last value on the series
    # that use the same mappings as the first) and form a new range that is contiguous.
    # The range generated from the breaking point onwards may or not be contiguous,
    # so we do that iteratively until EVERY RANGE in our list is contiguous.

    # When every range is contiguous, we only need to check the mapping for the FIRST
    # value of every range, because any other will be greater. This reduces the number
    # of checks we need to do from billions to some dozens.
    i = 0
    while i < len(range_list):
        ith_range = range_list[i]
        breaking_point = ith_range.find_breaking_point(full_mapper)
        if breaking_point is None:
            i += 1
        else:
            ith_reduced = SeedRange(ith_range.start, breaking_point)
            next_range = SeedRange(breaking_point + 1, ith_range.end)
            # Remove current ith
            range_list.pop(i)
            # Place new in its position
            range_list.insert(i, ith_reduced)
            # Place next
            range_list.insert(i + 1, next_range)
            i += 1
    lowest = full_mapper.map_source_to_dest(range_list[0].start)
    for seed_range in range_list:
        this_seed_lowest = full_mapper.map_source_to_dest(seed_range.start)
        if this_seed_lowest < lowest:
            lowest = this_seed_lowest
    print(lowest)
