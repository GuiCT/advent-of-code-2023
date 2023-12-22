lines = readlines("input.txt")
sequences = Vector{Vector{Int}}()
for i in eachindex(lines)
  this_line_numbers = String.(split(lines[i], " "))
  this_sequence_vector = zeros(Int, length(this_line_numbers))
  for j in eachindex(this_line_numbers)
    this_sequence_vector[j] = parse(Int, this_line_numbers[j])
  end
  push!(sequences, this_sequence_vector)
end

function predict(original_sequence::Vector{Int})
  decompositions = Vector{Vector{Int}}()
  push!(decompositions, original_sequence)
  while !iszero(decompositions[end])
    push!(decompositions, diff(decompositions[end]))
  end
  next_decomposition_values = zeros(Int, length(decompositions))
  number_of_decompositions = length(next_decomposition_values)
  for i in number_of_decompositions-1:-1:1
    next_decomposition_values[i] = decompositions[i][end] + next_decomposition_values[i + 1]
  end
  return next_decomposition_values[1]
end

sum = 0
for i in eachindex(sequences)
  prediction = predict(sequences[i])
  sum = sum + prediction
end
print(sum)

# ============================== SECOND PART ==============================

function predict_backwards(original_sequence::Vector{Int})
  decompositions = Vector{Vector{Int}}()
  push!(decompositions, original_sequence)
  while !iszero(decompositions[end])
    push!(decompositions, diff(decompositions[end]))
  end
  previous_decomposition_values = zeros(Int, length(decompositions))
  number_of_decompositions = length(previous_decomposition_values)
  for i in number_of_decompositions-1:-1:1
    previous_decomposition_values[i] = decompositions[i][1] - previous_decomposition_values[i + 1]
  end
  return previous_decomposition_values[1]
end

sum = 0
for i in eachindex(sequences)
  backwards_prediction = predict_backwards(sequences[i])
  sum = sum + backwards_prediction
end
print(sum)