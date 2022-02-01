package main

import (
	"errors"
	"fmt"
)

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
	// fmt.Printf("parse: %s\n\n", tok.token)
	switch tok.token {
	case CONST, LET:
		{
			ident, _ := p.nextOfType(IDENTIFIER)
			equal, _ := p.nextOfType(EQUAL)
			value, err := p.untilEndOfStatement()
			if err != nil {
				panic(err)
			}

			for _, t := range value {
				fmt.Println("t: ", t.text, t.token)
			}
			fmt.Printf("current: %s -> ident: %s '%s' -> equal: %s\n\n", tok.token, ident.token, ident.text, equal.token)
			// return Span{line: tok.span.line, column: tok.span.column}, VARIABLE, Var{variable_type: CONST, value: 10}
		}
	}
	return Span{}, 0, ""
}

// returns next of type and advances parser
func (p *Parser) nextOfType(t Token) (TokenEntry, error) {
	for i := int(p.index); i < len(p.tokens); i++ {
		tok := p.tokens[i]
		if i <= len(p.tokens) {
			p.index++

			if tok.token == t {
				return tok, nil
			}
		}
		// TODO: add code for determining if advanced past end of statement

	}

	return TokenEntry{}, errors.New("none found")
}

// returns next token and advances parser
func (p *Parser) next() (TokenEntry, error) {
	if int(p.index+1) <= len(p.tokens) {
		current := p.index
		return p.tokens[current], nil
	}

	return TokenEntry{}, errors.New("out of bounds")
}

func (p *Parser) untilEndOfStatement() ([]TokenEntry, error) {
	var toks []TokenEntry
	for i := int(p.index); i < len(p.tokens); i++ {
		tok := p.tokens[i]
		if i <= len(p.tokens) {
			p.index++

			if tok.token == LNBREAK || tok.token == SEMI {
				break
			}

			toks = append(toks, tok)
		}
		// TODO: add code for determining if advanced past end of statement
	}

	if len(toks) == 0 {
		return make([]TokenEntry, 0), errors.New("none found")
	}

	return toks, nil
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
	}
}

func Parse(lexer *Lexer) *Parser {
	return &Parser{
		index:  0,
		tokens: lexer.tokens,
	}
}
