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

int getNumberEnd(char matrix[COLUMNS], int start) {
  int counter = start;

  // If first char is not a digit, then this is not even a number.
  if (!isdigit(matrix[counter])) {
    return -1;
  }

  // Otherwise, just increment until we reach a char that is not a digit
  while ((counter < COLUMNS) && isdigit(matrix[counter])) {
    counter++;
  }

  return counter;
}

int getNumberStart(char matrix[COLUMNS], int pos) {
  int counter = pos;

  // If first char is not a digit, then this is not even a number.
  if (!isdigit(matrix[counter])) {
    return -1;
  }

  // Otherwise, just decrement until we reach a char that is not a digit
  while ((counter >= 0) && isdigit(matrix[counter])) {
    counter--;
  }

  return counter + 1;
}

int neighboringNumberValue(char matrix[LINES][COLUMNS], int line, int column,
                           int *numberEndReference) {
  int numberStart = getNumberStart(matrix[line], column);
  int numberEnd = getNumberEnd(matrix[line], numberStart);
  *numberEndReference = numberEnd;
  char *str = (char *)(malloc((numberEnd - numberStart + 1) * sizeof(char)));
  strncpy(str, matrix[line] + numberStart, numberEnd - numberStart);
  str[numberEnd - numberStart] = 0;
  int value = atoi(str);
  free(str);
  return value;
}

int getGearRatio(char matrix[LINES][COLUMNS], int line, int column) {
  int lineBefore = line - 1;
  int lineAfter = line + 1;
  int columnBefore = column - 1;
  int columnAfter = column + 1;
  int start = max(0, columnBefore);
  int end = min(columnAfter, COLUMNS);
  unsigned int neighboring = 0;
  int product = 1;
  int numberEndReference;

  // If there is a line before
  if (lineBefore >= 0) {
    int columnIt = start;
    while (columnIt <= end) {
      if (isdigit(matrix[lineBefore][columnIt])) {
        product *= neighboringNumberValue(matrix, lineBefore, columnIt,
                                          &numberEndReference);
        neighboring++;
        columnIt = numberEndReference;
      } else {
        columnIt++;
      }
    }
  }

  // For the SAME line
  if (columnBefore >= 0) {
    if (isdigit(matrix[line][columnBefore])) {
      product *=
          neighboringNumberValue(matrix, line, columnBefore, &numberEndReference);
      neighboring++;
    }
  }
  if (columnAfter < COLUMNS) {
    if (isdigit(matrix[line][columnAfter])) {
      product *= neighboringNumberValue(matrix, line, columnAfter,
                                        &numberEndReference);
      neighboring++;
    }
  }

  // If there is a line after
  if (lineAfter < LINES) {
    int columnIt = start;
    while (columnIt <= end) {
      if (isdigit(matrix[lineAfter][columnIt])) {
        product *= neighboringNumberValue(matrix, lineAfter, columnIt,
                                          &numberEndReference);
        neighboring++;
        columnIt = numberEndReference;
      } else {
        columnIt++;
      }
    }
  }

  if (neighboring == 2) {
    return product;
  } else {
    return -1;
  }
}

int main(void) {
  FILE *file;
  file = fopen("input.txt", "r");
  if (file == NULL) {
    exit(EXIT_FAILURE);
  }

  char *line;
  size_t lineLength = 0;
  int bytesRead = 0;
  char matrix[LINES][COLUMNS];
  int currentLine = 0;

  while ((bytesRead = getline(&line, &lineLength, file)) != -1) {
    if (currentLine >= LINES) {
      printf("Something is wrong. Matrix size is not enough for this...\n");
      exit(EXIT_FAILURE);
    }
    strncpy(matrix[currentLine++], line, COLUMNS);
  }

  unsigned int sum = 0;
  for (int line = 0; line < LINES; line++) {
    int column = 0;
    while (column < COLUMNS) {
      if (matrix[line][column] == '*') {
        int product = getGearRatio(matrix, line, column);
        if (product != -1) {
          sum += product;
        }
      }
      column++;
    }
  }

  printf("%d\n", sum);

  return 0;
}
