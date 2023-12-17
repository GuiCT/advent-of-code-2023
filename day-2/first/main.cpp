#include <ctype.h>
#include <fstream>
#include <inttypes.h>
#include <iostream>
#include <sstream>
#include <stdlib.h>
#include <string>
#include <vector>

#define MAX_REDS 12
#define MAX_GREENS 13
#define MAX_BLUES 14

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

bool isGameValid(Result game) {
  for (size_t i = 0; i < game.red.size(); i++) {
    if (game.red[i] > MAX_REDS) {
      return false;
    }
  }

  for (size_t i = 0; i < game.green.size(); i++) {
    if (game.green[i] > MAX_GREENS) {
      return false;
    }
  }

  for (size_t i = 0; i < game.blue.size(); i++) {
    if (game.blue[i] > MAX_BLUES) {
      return false;
    }
  }

  return true;
}

int main(void) {
  std::ifstream inputFile;
  inputFile.open("input.txt");
  std::size_t lineNumber = 0;
  std::size_t sum = 0;
  for (std::string line; std::getline(inputFile, line);) {
    Result res = parseLine(line);
    if (isGameValid(res)) {
      sum += res.id;
    }
  }
  std::cout << sum << std::endl;
  return 0;
}