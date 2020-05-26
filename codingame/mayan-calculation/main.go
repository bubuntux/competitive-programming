package main

import (
	"bytes"
	"fmt"
	"math"
	"os"
	"strconv"
)

func readNumbers() []string {
	var L, H int
	fmt.Scan(&L, &H)

	buffers := make([]bytes.Buffer, 20)
	for h := 1; h <= H; h++ {
		var numeral string
		fmt.Scan(&numeral)
		for l := 0; l < 20; l++ {
			startIndex := l * L
			substring := numeral[startIndex : startIndex+L]
			buffers[l].WriteString(substring)
			if h != H {
				buffers[l].WriteString("\n")
			}
		}
	}

	numbers := make([]string, 20)
	for index, buffer := range buffers {
		numbers[index] = buffer.String()
	}
	return numbers
}

func readOperator(stringToNumber map[string]int) int {
	var input bytes.Buffer
	var S int
	var n []int
	fmt.Scan(&S)
	fmt.Fprintln(os.Stderr, "\n"+strconv.Itoa(S))
	for i := 1; i <= S; i++ {
		var line string
		fmt.Scan(&line)
		input.WriteString(line)
		fmt.Fprintln(os.Stderr, line)
		if val, ok := stringToNumber[input.String()]; ok {
			n = append(n, val)
			input.Reset()
		}else{
			input.WriteString("\n")
		}
	}
	pow := len(n)-1
	count := 0
	for i := 0; i < len(n); i++ {
		count += int(float64(n[i]) * math.Pow(20, float64(pow)))
		pow--
	}
	return count
}

func operation(value1 int, value2 int) int {
	var operation string
	fmt.Scan(&operation)

	var value int
	switch operation {
	case "+":
		value = value1 + value2
		break
	case "-":
		value = value1 - value2
		break
	case "*":
		value = value1 * value2
		break
	case "/":
		value = value1 / value2
		break
	}
	return value
}

func main() {
	numberToString := readNumbers()
	stringToNumber := make(map[string]int)
	for i := 0; i < 20; i++ {
		n := numberToString[i]
		stringToNumber[n] = i
		fmt.Fprintln(os.Stderr, "\n"+strconv.Itoa(i)+"\n"+n)
	}

	value1 := readOperator(stringToNumber)
	fmt.Fprintln(os.Stderr, "input1\n"+strconv.Itoa(value1)+"\n")

	value2 := readOperator(stringToNumber)
	fmt.Fprintln(os.Stderr, "input2\n"+strconv.Itoa(value2)+"\n")

	value := operation(value1, value2)

	fmt.Fprintln(os.Stderr, "\n"+strconv.Itoa(value))

	var reminders []int
	for value > 20 {
		reminder := value % 20
		reminders = append(reminders, reminder)
		fmt.Fprintln(os.Stderr, "reminder: "+strconv.Itoa(reminder))
		value = value / 20
	}
	reminders = append(reminders, value)
	fmt.Fprintln(os.Stderr, "value: "+strconv.Itoa(value))

	var buffer bytes.Buffer
	for i := len(reminders) - 1; i >= 0; i-- {
		buffer.WriteString(numberToString[reminders[i]])
		buffer.WriteString("\n")
	}

	fmt.Println(buffer.String())
}
