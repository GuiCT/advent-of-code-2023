package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

// Source: https://siongui.github.io/2017/06/03/go-find-lcm-by-gcd/
// greatest common divisor (GCD) via Euclidean algorithm
func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// find Least Common Multiple (LCM) via GCD
func LCM(a, b int, integers ...int) int {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
		result = LCM(result, integers[i])
	}

	return result
}

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
	currents := make([]string, 0)

	for i := 2; i < len(fileLines); i++ {
		key := fileLines[i][0:3]
		if key[2] == 'A' {
			currents = append(currents, key)
		}
		first := fileLines[i][7:10]
		second := fileLines[i][12:15]
		var stringArr [2]string
		stringArr[0] = first
		stringArr[1] = second
		stringMap[key] = stringArr
	}

	numActions := len(actions)
	actionsCounterForEachStartPoint := make([]int, len(currents))
	for i, _ := range currents {
		actionsCounterForEachStartPoint[i] = 0
		for currents[i][2] != 'Z' {
			currents[i] = stringMap[currents[i]][actions[actionsCounterForEachStartPoint[i]%numActions]]
			actionsCounterForEachStartPoint[i]++
		}
	}

	fmt.Printf("Total move count: %d", LCM(actionsCounterForEachStartPoint[0], actionsCounterForEachStartPoint[1], actionsCounterForEachStartPoint[2:]...))
}
