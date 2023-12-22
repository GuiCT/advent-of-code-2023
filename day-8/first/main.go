package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	fileTextBytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal("Error when opening input file")
	}

	fileText := string(fileTextBytes)
	fileLines := strings.Split(fileText, "\n")
	actionsStr := fileLines[0]
	actions := make([]int8, len(strings.Split(actionsStr, "")))

	for i, v := range actionsStr {
		if v == 'L' {
			actions[i] = 0
		} else {
			actions[i] = 1
		}
	}

	stringMap := make(map[string][2]string)

	for i := 2; i < len(fileLines); i++ {
		key := fileLines[i][0:3]
		first := fileLines[i][7:10]
		second := fileLines[i][12:15]
		var stringArr [2]string
		stringArr[0] = first
		stringArr[1] = second
		stringMap[key] = stringArr
	}

	numActions := len(actions)
	actionsCounter := 0
	current := "AAA"
	toBeFound := "ZZZ"
	for current != toBeFound {
		current = stringMap[current][actions[actionsCounter%numActions]]
		actionsCounter++
	}
	fmt.Printf("Total move count: %d", actionsCounter)
}
