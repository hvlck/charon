package main

import (
	"log"
	"os"
)

func main() {
	file, _ := os.Open("./test.chrn")

	l := Lex(file)
	l.Start()

	l.removeWhitespace()
	for i, t := range l.tokens {
		log.Printf("tok #%v :: '%s' of token type %v on ln %v:%v", i, t.text, t.token, t.span.line, t.span.column)
	}

	p := Parse(l)
	p.Start()

	for i, t := range p.ast {
		log.Printf("node #%v :: '%s' of node type %v on ln %v:%v", i, t.body, t.node_type, t.span.line, t.span.column)
	}

	c := NewChunk()
	c.Write(OP_RETURN)
	c.Write(OP_RETURN)
	c.Write(OP_RETURN)
	c.Write(OP_RETURN)
	c.Disassemble("chunk")
}
