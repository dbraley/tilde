package main

import (
	"fmt"
	"github.com/c-bata/go-prompt"
	"os"
	"strings"
)

func completer(d prompt.Document) []prompt.Suggest {
	s := []prompt.Suggest{
		{Text: "zork", Description: "Enter a (limited) zork game"},
		{Text: "exit", Description: "Exit the program"},
		{Text: "nothing", Description: "Do nothing"},
	}
	return prompt.FilterHasPrefix(s, d.GetWordBeforeCursor(), true)
}

func main() {
	fmt.Println("Please select an option.")
	t := prompt.Input("> ", completer)
	switch t {
	case "zork":
		zork()
	case "exit":
		os.Exit(0)
	default:
		fmt.Println("Not sure what to do with " + t)
	}
}

func zork() {
	g := &game{}
	p := prompt.New(
		g.executor,
		g.completer,
		prompt.OptionTitle("Zorks!"),
		prompt.OptionPrefix(">>> "),
		prompt.OptionInputTextColor(prompt.Yellow),
		prompt.OptionSetExitCheckerOnInput(g.ExitChecker),
	)
	p.Run()
}

type game struct {
	done bool
}

func (g *game) executor(s string) {
	if s == "go north" {
		fmt.Println("You win!")
		g.done = true
		return
	}
}

func (g *game) completer(d prompt.Document) []prompt.Suggest {
	args := strings.Split(d.Text, " ")
	var s []prompt.Suggest
	switch args[0] {
	case "go":
		s = []prompt.Suggest{
			{"north", "Head North"},
			{"south", "Head South"},
			{"east", "Head East"},
			{"west", "Head West"},
		}
	default:
		s = []prompt.Suggest{
			{"go", "Go north/south/east/west"},
		}
	}
	return prompt.FilterHasPrefix(s, d.GetWordBeforeCursor(), true)
}

func (g *game) ExitChecker(string, bool) bool {
	return g.done
}
