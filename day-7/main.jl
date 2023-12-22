using LinearAlgebra: dot
using Printf

char_values = Dict(
  'A' => 13,
  'K' => 12,
  'Q' => 11,
  'J' => 10,
  'T' => 9,
  '9' => 8,
  '8' => 7,
  '7' => 6,
  '6' => 5,
  '5' => 4,
  '4' => 3,
  '3' => 2,
  '2' => 1,
)

function get_card_type(card_string::String)
  char_dict = Dict{Char,Int}()
  for char in card_string
    if haskey(char_dict, char)
      char_dict[char] = char_dict[char] + 1
    else
      char_dict[char] = 1
    end
  end

  if length(char_dict) == 1
    7
  elseif length(char_dict) == 2
    four_times_keys = filter(((k, v),) -> v == 4, char_dict)
    if length(four_times_keys) == 1
      6
    else
      5
    end
  elseif length(char_dict) == 3
    two_times_keys = filter(((k, v),) -> v == 2, char_dict)
    if length(two_times_keys) == 2
      3
    else
      4
    end
  elseif length(char_dict) == 4
    2
  elseif length(char_dict) == 5
    1
  end
end

# Read every line
lines = readlines("input.txt")
line_count = length(lines)
matrix_of_cards = zeros(Int64, line_count, 7)
# For each matrix row
for (i, line) in enumerate(lines)
  # Split card chars and bid value
  card, bid = String.(split(line, ' '))
  # Type will be the first sort key
  type_of_card = get_card_type(card)
  matrix_of_cards[i, 1] = type_of_card
  # Next sort keys will use each letter value
  for (j, letter) in enumerate(card)
    matrix_of_cards[i, j+1] = char_values[letter]
  end
  # Parse bid as the last value
  matrix_of_cards[i, 7] = parse(Int64, bid)
end

# Stable sort, column by column (ignoring bid value)
matrix_of_cards = sortslices(matrix_of_cards, dims=1, by=(x -> tuple(x[1:end-1]...)));
# Dot product to calculate the result
last_column = matrix_of_cards[:, end];
ranks = 1:line_count;
@printf("Result is %d\n", dot(last_column, ranks))