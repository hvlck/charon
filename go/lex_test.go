package main

import (
	"os"
	"testing"
)

func LexTest(t *testing.T) {
	file, _ := os.Open("./test.chrn")

	Lex(file)
}
