#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINES 140
#define COLUMNS 140

int max(int a, int b) {
  if (a > b) {
    return a;
  } else {
    return b;
  }
}

int min(int a, int b) {
  if (a < b) {
    return a;
  } else {
    return b;
  }
}

size_t getNumberEnd(char matrix[COLUMNS], size_t start) {
  size_t counter = start;

  // If first char is not a digit, then this is not even a number.
  if (!isdigit(matrix[counter++])) {
    return -1;
  }

  // Otherwise, just increment until we reach a char that is not a digit
  while ((counter < COLUMNS) && isdigit(matrix[counter])) {
    counter++;
  }

  return counter;
}

bool haveNeighboringSymbol(char matrix[LINES][COLUMNS], int line,
                           int columnStart, int columnEnd) {
  int lineBefore = line - 1;
  int lineAfter = line + 1;
  int columnBefore = columnStart - 1;
  int columnAfter = columnEnd;
  int start = max(0, columnBefore);
  int end = min(columnAfter, COLUMNS);

  // If there is a line before
  if (lineBefore >= 0) {
    for (size_t column = start; column <= end; column++) {
      if (matrix[lineBefore][column] != '.') {
        return true;
      }
    }
  }

  // For the SAME line
  if (columnBefore >= 0) {
    if (matrix[line][columnBefore] != '.') {
      return true;
    }
  }
  if (columnAfter < COLUMNS) {
    if (matrix[line][columnAfter] != '.') {
      return true;
    }
  }

  // If there is a line after
  if (lineAfter < LINES) {
    for (size_t column = start; column <= end; column++) {
      if (matrix[lineAfter][column] != '.') {
        return true;
      }
    }
  }

  return false;
}

int main(void) {
  FILE *file;
  file = fopen("input.txt", "r");
  if (file == NULL) {
    exit(EXIT_FAILURE);
  }

  char *line;
  size_t lineLength = 0;
  size_t bytesRead = 0;
  char matrix[LINES][COLUMNS];
  size_t currentLine = 0;

  while ((bytesRead = getline(&line, &lineLength, file)) != -1) {
    if (currentLine >= LINES) {
      printf("Something is wrong. Matrix size is not enough for this...\n");
      exit(EXIT_FAILURE);
    }
    strncpy(matrix[currentLine++], line, COLUMNS);
  }

  unsigned int sum = 0;
  for (size_t line = 0; line < LINES; line++) {
    size_t column = 0;
    while (column < COLUMNS) {
      int end = getNumberEnd(matrix[line], column);
      if (end != -1) {
        if (haveNeighboringSymbol(matrix, line, column, end)) {
          char* str = (char*)(malloc((end - column + 1) * sizeof(char)));
          strncpy(str, matrix[line] + column, end - column);
          sum += atoi(str);
        }
        column = end;
      } else {
        column++;
      }
    }
  }

  printf("%d", sum);

  return 0;
}
