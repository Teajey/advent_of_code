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

var word2Digit = map[string]rune{
	"zero":  '0',
	"one":   '1',
	"two":   '2',
	"three": '3',
	"four":  '4',
	"five":  '5',
	"six":   '6',
	"seven": '7',
	"eight": '8',
	"nine":  '9',
}

func containsDigit(str string) bool {
	for _, c := range str {
		if unicode.IsDigit(c) {
			return true
		}
	}
	return false
}

func findDigitAsWord(str string) rune {
	for i := 3; i <= 5 && i <= len(str); i++ {
		maybeWord := str[0:i]
		if containsDigit(maybeWord) {
			break
		}
		fmt.Println("Maybe a word?", maybeWord)
		d, ok := word2Digit[maybeWord]
		if ok {
			fmt.Println("It's a word!")
			return d
		}
	}
	return rune(0)
}

func firstAndLastDigit(line string) (a rune, b rune) {
	firstSet := false
	for i := 0; i < len(line); i++ {
		char := rune(line[i])
		if !unicode.IsDigit(char) {
			digit := findDigitAsWord(line[i:])
			if digit == rune(0) {
				continue
			}
			char = digit
		}
		if firstSet == false {
			fmt.Println("Found start digit:", string(char))
			a = char
			b = char
			firstSet = true
		} else {
			fmt.Println("Found new end digit:", string(char))
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
		line = line[:len(line)-1]
		fmt.Println("Reading line:", line)
		a, b := firstAndLastDigit(line)
		i, err := strconv.Atoi(string(a) + string(b))
		if err != nil {
			log.Fatalln(err)
		}
		n += uint(i)
	}
	fmt.Printf("%v\n", n)
}
