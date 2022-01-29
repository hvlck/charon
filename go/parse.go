package main

import "fmt"

type Node int

const (
	END = iota

	FN
	BINARY_EXPR
	UNARY_EXPR
	FN_CALL
	VARIABLE
)

var nodes = []string{
	FN:          "FN",
	BINARY_EXPR: "BINARY_EXPR",
	UNARY_EXPR:  "UNARY_EXPR",
	FN_CALL:     "FN_CALL",
	VARIABLE:    "VAR",
}

const (
	CONSTANT_VAR = iota
	LET_VAR
)

type VAR_TYPE int

type Var struct {
	variable_type VAR_TYPE
	identifier    string
	value         interface{}
}

type ASTNode struct {
	body      interface{}
	node_type Node
	span      Span
}

type Parser struct {
	index  uint
	tokens []TokenEntry
	ast    []ASTNode
}

func (p *Parser) Parse(tok TokenEntry) (Span, Node, interface{}) {
	fmt.Printf("parse: %s\n\n", tok.token)
	switch tok.token {
	case CONST | LET:
		{
			fmt.Println("CONST OR LET")
			ident := p.next()
			equal := p.next()
			fmt.Printf("current: %s || ident: %s || equal: %s\n\n", tok.token, ident.token, equal.token)
		}
	}
	return Span{}, 0, ""
}

// returns next token and advances parser
func (p *Parser) next() TokenEntry {
	index := p.index
	p.index++
	return p.tokens[index]
}

// returns next n tokens and advances parser
// func (p *Parser) next_nth(n uint) []TokenEntry {
// 	index := p.index
// 	p.index += n
// }

func (p *Parser) unadvance(n uint) {
	p.index -= n
}

func (p *Parser) Start() {
	for _, t := range p.tokens {
		span, node, body := p.Parse(t)
		p.ast = append(p.ast, ASTNode{span: span, node_type: node, body: body})
		p.index++
	}
}

func Parse(lexer *Lexer) *Parser {
	return &Parser{
		index:  0,
		tokens: lexer.tokens,
	}
}
