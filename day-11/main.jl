using SparseArrays

if length(ARGS) == 0
  @error "Part 1 or 2, moron? Specify args."
  exit(1)
end
increment = if ARGS[1] == "1"
  1
else ARGS[1] == "2"
  999999
end
println(increment)

lines = String.(readlines("input.txt"))
line_count = length(lines)
line_length = length(lines[1])

lines_indexes = Int[]
columns_indexes = Int[]
# Populate positionsDict
for i in eachindex(lines)
  for j in eachindex(lines[i])
    if lines[i][j] == '#'
      push!(lines_indexes, i)
      push!(columns_indexes, j)
    end
  end
end
@assert length(lines_indexes) == length(columns_indexes)
num_indexes = length(lines_indexes)

empty_lines = filter(i -> !(i in unique(lines_indexes)), collect(1:line_count))
empty_columns = filter(j -> !(j in unique(columns_indexes)), collect(1:line_length))

positions = collect(zip(lines_indexes, columns_indexes))
num_positions = length(positions)
global sum_of_shortest_paths = 0
for i in 1:num_positions-1
  for j in i+1:num_positions
    column_min = min(positions[i][2], positions[j][2])
    column_max = max(positions[i][2], positions[j][2])
    line_min = min(positions[i][1], positions[j][1])
    line_max = max(positions[i][1], positions[j][1])
    original_distance = sum(abs.(positions[i] .- positions[j]))
    extra_dist = length(filter(l -> Base.isbetween(line_min, l, line_max), empty_lines)) + length(filter(c -> Base.isbetween(column_min, c, column_max), empty_columns))
    extra_dist *= increment
    global sum_of_shortest_paths += (original_distance + extra_dist)
  end
end
println(sum_of_shortest_paths)