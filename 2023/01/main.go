package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"unicode"
)

func firstAndLastDigit(line string) (a rune, b rune) {
	firstSet := false
	for _, char := range line {
		if !unicode.IsDigit(char) {
			continue
		}
		if firstSet == false {
			a = char
			b = char
			firstSet = true
		} else {
			b = char
		}
	}
	return
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	var n uint
	for {
		line, err := reader.ReadString('\n')
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalln(err)
		}
		a, b := firstAndLastDigit(line)
		i, err := strconv.Atoi(string(a) + string(b))
		if err != nil {
			log.Fatalln(err)
		}
		n += uint(i)
	}
	fmt.Printf("%v\n", n)
}
