package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

const SYMBOLS = "#$%&*+-/=@"

func scanForNum(str string, from int) (start int, end int) {
	start = -1
	end = -1
	for i, r := range str[from:] {
		startFound := start != -1
		isDigit := unicode.IsDigit(r)
		if !startFound && isDigit {
			start = from + i
		} else if startFound && !isDigit {
			break
		}
		end = from + i + 1
	}
	return
}

func clampedStrSlice(str string, start int, end int) string {
	if start < 0 {
		start = 0
	}
	length := len(str)
	if end > length {
		end = length
	}
	if start > end {
		start = end
	}
	return str[start:end]
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	var n int
	var prevLine string
	var currentLine string
	for {
		nextLine, err := reader.ReadString('\n')
		if err != nil && err != io.EOF {
			log.Fatalln(err)
		}
		if nextLine != "" {
			nextLine = nextLine[:len(nextLine)-1]
		}
		if currentLine == "" {
			currentLine = nextLine
			continue
		}

		for i := 0; ; {
			if currentLine == "" {
				break
			}
			start, end := scanForNum(currentLine, i)
			if start == -1 || start == end {
				break
			}
			i = end
			num, err := strconv.Atoi(currentLine[start:end])
			if err != nil {
				log.Fatalln("Couldn't parse num:", err)
			}

			fmt.Println("\nnum:", num)

			if prevLine != "" {
				prevWindow := clampedStrSlice(prevLine, start-1, end+1)
				fmt.Println("prevWindow:", prevWindow)
				if strings.ContainsAny(prevWindow, SYMBOLS) {
					n += num
					continue
				}
			}

			if start-1 >= 0 {
				leading := currentLine[start-1 : start]
				fmt.Println("leading:", leading)
				if strings.ContainsAny(leading, SYMBOLS) {
					n += num
					continue
				}
			}

			if end < len(currentLine) {
				trailing := currentLine[end : end+1]
				fmt.Println("trailing:", trailing)
				if strings.ContainsAny(trailing, SYMBOLS) {
					n += num
					continue
				}
			}

			if nextLine != "" {
				nextWindow := clampedStrSlice(nextLine, start-1, end+1)
				fmt.Println("nextWindow:", nextWindow)
				if strings.ContainsAny(nextWindow, SYMBOLS) {
					n += num
					continue
				}
			}

			if end >= len(currentLine)-1 {
				break
			}
		}

		prevLine = currentLine
		currentLine = nextLine
		if currentLine == "" {
			break
		}
	}
	fmt.Println(n)
}
