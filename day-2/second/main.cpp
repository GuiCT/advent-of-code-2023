#include <algorithm>
#include <ctype.h>
#include <fstream>
#include <inttypes.h>
#include <iostream>
#include <sstream>
#include <stdlib.h>
#include <string>
#include <vector>

typedef struct m_result {
  std::size_t id;
  std::vector<uint> red;
  std::vector<uint> green;
  std::vector<uint> blue;
} Result;

Result parseLine(std::string line) {
  Result res;
  res.red = std::vector<uint>();
  res.green = std::vector<uint>();
  res.blue = std::vector<uint>();
  line = line.substr(5);
  std::stringstream lineStream(line);
  std::string word;
  std::size_t currentVal = -1;
  while (lineStream >> word) {
    char lastChar = word[word.length() - 1];
    if (lastChar == ':') {
      res.id = std::stoi(word.substr(0, word.length() - 1));
      continue;
    }

    if (lastChar == ',' || lastChar == ';' || !isdigit(lastChar)) {
      if (word[0] == 'r') {
        res.red.push_back(currentVal);
      } else if (word[0] == 'g') {
        res.green.push_back(currentVal);
      } else if (word[0] == 'b') {
        res.blue.push_back(currentVal);
      }
      continue;
    }

    char *p;
    long converted = strtol(word.data(), &p, 10);
    if (!(*p)) {
      currentVal = converted;
    } else {
      std::cout << "xiiiiiiiiii" << std::endl;
    }
  }
  return res;
}

std::size_t getPowerOfSet(Result game) {
  // For each game, the minimum for each color that makes the game feasable is
  // the maximum value of each.

  uint maxRed;
  uint maxGreen;
  uint maxBlue;
  std::vector<uint>::iterator iterator;
  iterator = std::max_element(game.red.begin(), game.red.end());
  if (iterator != game.red.end()) {
    maxRed = *iterator;
  }
  iterator = std::max_element(game.green.begin(), game.green.end());
  if (iterator != game.green.end()) {
    maxGreen = *iterator;
  }
  iterator = std::max_element(game.blue.begin(), game.blue.end());
  if (iterator != game.blue.end()) {
    maxBlue = *iterator;
  }
  
  return maxRed * maxGreen * maxBlue;
}

int main(void) {
  std::ifstream inputFile;
  inputFile.open("input.txt");
  std::size_t lineNumber = 0;
  std::size_t sum = 0;
  for (std::string line; std::getline(inputFile, line);) {
    Result res = parseLine(line);
    sum += getPowerOfSet(res);
  }
  std::cout << sum << std::endl;
  return 0;
}