using LinearAlgebra: dot
using Printf

char_values = Dict(
  'A' => 13,
  'K' => 12,
  'Q' => 11,
  'T' => 9,
  '9' => 8,
  '8' => 7,
  '7' => 6,
  '6' => 5,
  '5' => 4,
  '4' => 3,
  '3' => 2,
  '2' => 1,
  # Lowest value now!
  'J' => 0,
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
  joker_count = if haskey(char_dict, 'J')
    char_dict['J']
  else
    0
  end
  # If we have jokers
  if joker_count > 0
    # And there is not only jokers
    if length(char_dict) > 1
      # Eliminate the joker_count from the dictionary
      pop!(char_dict, 'J')
      # Increment it on the highest count char that is not a joker
      highest_count_char = findmax(char_dict)[2]
      char_dict[highest_count_char] += joker_count
    end
  end
  # Proceed as usual
  char_dict_entry_count = length(char_dict)

  if char_dict_entry_count == 1
    7
  elseif char_dict_entry_count == 2
    four_times_keys = filter(((k, v),) -> v == 4, char_dict)
    if length(four_times_keys) == 1
      6
    else
      5
    end
  elseif char_dict_entry_count == 3
    two_times_keys = filter(((k, v),) -> v == 2, char_dict)
    if length(two_times_keys) == 2
      3
    else
      4
    end
  elseif char_dict_entry_count == 4
    2
  elseif char_dict_entry_count == 5
    1
  end
end

# Read every line
lines = readlines("day-7/input.txt")
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

matrix_of_cards

# Stable sort, column by column (ignoring bid value)
matrix_of_cards = sortslices(matrix_of_cards, dims=1, by=(x -> tuple(x[1:end-1]...)));

matrix_of_cards
# Dot product to calculate the result
last_column = matrix_of_cards[:, end];
ranks = 1:line_count;
@printf("Result is %d\n", dot(last_column, ranks))