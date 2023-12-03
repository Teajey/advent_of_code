package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

func parseCube(str string) (int, string) {
	items := strings.Split(str, " ")
	if len(items) != 2 {
		log.Fatalln("cube items not 2: ", items)
	}
	amount, err := strconv.Atoi(items[0])
	if err != nil {
		log.Fatalln("Amount is not int:", err)
	}
	color := items[1]
	return amount, color
}

type reveal struct {
	red   int
	green int
	blue  int
}

func parseReveal(str string) (rev reveal) {
	cubeStrs := strings.Split(str, ", ")
	for _, r := range cubeStrs {
		amount, color := parseCube(r)
		switch color {
		case "red":
			rev.red = amount
		case "green":
			rev.green = amount
		case "blue":
			rev.blue = amount
		default:
			log.Fatalln("Invalid color:", color)
		}
	}
	return
}

func parseGame(str string) (id int, reveals []reveal) {
	idAndGame := strings.Split(str, ": ")
	if len(idAndGame) != 2 {
		log.Fatalln("Not two `: ` items~~", idAndGame)
	}
	idStrs := strings.Split(idAndGame[0], " ")
	if len(idStrs) != 2 {
		log.Fatalln("Not two ` ` items~~", idStrs)
	}
	id, err := strconv.Atoi(idStrs[1])
	if err != nil {
		log.Fatalln("Game ID isnt integer:", err)
	}
	revealStrs := strings.Split(idAndGame[1], "; ")
	for _, r := range revealStrs {
		reveals = append(reveals, parseReveal(r))
	}
	return
}

func minimumSet(game []reveal) (r int, g int, b int) {
	for _, rev := range game {
		if rev.red > r {
			r = rev.red
		}
		if rev.green > g {
			g = rev.green
		}
		if rev.blue > b {
			b = rev.blue
		}
	}
	return r, g, b
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	if len(os.Args) != 4 {
		log.Fatalln("Args must be len() == 4: ", os.Args)
	}
	var n int
	for {
		line, err := reader.ReadString('\n')
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalln(err)
		}
		line = line[:len(line)-1]
		fmt.Println(line)
		id, game := parseGame(line)
		fmt.Printf("game %v: %#v\n", id, game)
		r, g, b := minimumSet(game)
		n += r * g * b
	}
	fmt.Println(n)
}
