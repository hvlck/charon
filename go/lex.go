package main

import (
	"bufio"
	"io"
	"strings"
	"unicode"
)

type Token int

const (
	EOI = iota

	ADD
	SUB
	DIV
	MUL

	EQUAL

	LESS
	GREATER

	NUMBER
	STRING
	IDENTIFIER
	WHITESPACE
	LNBREAK
	SEMI

	CONST
	LET

	TRUE
	FALSE

	INVALID

	LBRACE
	RBRACE

	LPARAN
	RPARAN

	SPREAD
	PROPERTY

	FUNCTION

	TYPE
	STRUCT
	ENUM

	PIPE
	OR

	AMPERSAND
	AND

	COMMENT
	DOC_COMMENT
)

var tokens = []string{
	EOI: "EOI",

	ADD: "+",
	SUB: "-",
	MUL: "*",
	DIV: "/",

	EQUAL: "=",

	LESS:    "<",
	GREATER: ">",

	NUMBER:     "NUMBER",
	STRING:     "STRING",
	IDENTIFIER: "IDENTIFIER",
	WHITESPACE: "WHITESPACE",
	LNBREAK:    "LNBREAK",
	SEMI:       "SEMI",

	CONST: "CONST",
	LET:   "LET",

	TRUE:  "FALSE",
	FALSE: "FALSE",

	INVALID: "INVALID",

	LBRACE: "{",
	RBRACE: "}",

	LPARAN: "(",
	RPARAN: ")",

	SPREAD:   "...",
	PROPERTY: ".",

	FUNCTION: "FN",

	TYPE:   "TYPE",
	STRUCT: "STRUCT",
	ENUM:   "ENUM",

	PIPE: "|",
	OR:   "||",

	AMPERSAND: "&",
	AND:       "&&",

	COMMENT:     "COMMENT",
	DOC_COMMENT: "DOC_COMMENT",
}

type TokenList []Token

func (t Token) String() string {
	return tokens[t]
}

type TokenEntry struct {
	span  Span
	text  string
	token Token
}

type Lexer struct {
	span   Span
	reader *bufio.Reader
	tokens []TokenEntry
}

func (l *Lexer) Lex(rn rune) (Span, Token, string) {
	switch rn {
	case '+':
		return l.span, ADD, "+"
	case '-':
		return l.span, SUB, "-"
	case '*':
		return l.span, MUL, "*"
	case '/':
		{
			// normal comments don't work b/c of advancement bug
			doc_comment := l.nextIsSame(2, '/')

			if doc_comment {
				return l.span, DOC_COMMENT, "///"
			}

			l.unadvance(1)

			comment := l.nextIsSame(1, '/')

			if comment {
				return l.span, COMMENT, "//"
			}

			return l.span, DIV, "/"
		}
	case '=':
		return l.span, EQUAL, "="
	case '{':
		return l.span, LBRACE, "{"
	case '}':
		return l.span, RBRACE, "}"
	case '(':
		return l.span, LPARAN, "("
	case ')':
		return l.span, RPARAN, ")"
	case ';':
		return l.span, SEMI, ";"
	case '.':
		{
			next := l.nextIsSame(2, '.')
			if !next {
				return l.span, PROPERTY, "."
			}

			return l.span, SPREAD, "..."
		}
	default:
		if unicode.IsSpace(rn) {
			switch rn {
			case '\n' | '\r' | '\t':
				return l.span, LNBREAK, ""
			case ' ':
				return l.span, WHITESPACE, ""
			}
		} else if unicode.IsDigit(rn) {
			st := l.span

			l.unadvance(1)
			number := l.lexNumber()

			return st, NUMBER, number
		} else if unicode.IsLetter(rn) {
			st := l.span

			l.unadvance(1)
			ident := l.lexIdentifier()

			switch strings.ToLower(ident) {
			case "const":
				return st, CONST, "const"
			case "let":
				return st, LET, "let"
			case "true":
				return st, TRUE, "true"
			case "false":
				return st, FALSE, "false"
			case "fn":
				return st, FUNCTION, "fn"
			default:
				return st, IDENTIFIER, ident
			}
		}

		return l.span, WHITESPACE, string(rn)
	}
}

// parsing multi-column tokens

func (l *Lexer) nextIsSame(n int, char rune) bool {
	is_same := true
	for i := 0; i < n; i++ {
		r, _, err := l.reader.ReadRune()
		if err != nil {

		}

		if r != char {
			is_same = false
			break
		}
	}

	return is_same
}

func (l *Lexer) lexNumber() string {
	number := ""
	for {
		// fix err at some point
		r, _, _ := l.reader.ReadRune()

		l.span.column++

		if unicode.IsDigit(r) {
			number += string(r)
		} else {
			l.unadvance(1)
			break
		}
	}

	return number
}

func (l *Lexer) lexIdentifier() string {
	number := ""
	for {
		// fix err at some point
		r, _, _ := l.reader.ReadRune()

		l.span.column++

		if unicode.IsLetter(r) {
			number += string(r)
		} else {
			l.unadvance(1)
			break
		}
	}

	return number
}

// changing lexer position

func (l *Lexer) unadvance(n int) {
	for i := 0; i < n; i++ {
		if err := l.reader.UnreadRune(); err != nil {
			panic(err)
		}

		l.span.column--
	}
}

func (l *Lexer) Start() {
	for {
		rn, _, err := l.reader.ReadRune()

		if err != nil {
			if err == io.EOF {
				// needs to append spans and such
				l.tokens = append(l.tokens, TokenEntry{token: EOI, span: l.span, text: "EOI"})
			}

			break
			// error here...
		}

		if rn == '\n' || rn == '\r' {
			l.advanceLine()
		}

		l.span.column++

		sp, tok, str := l.Lex(rn)
		l.tokens = append(l.tokens, TokenEntry{span: sp, token: tok, text: str})
	}
}

func (l *Lexer) removeWhitespace() {
	var n []TokenEntry
	for _, v := range l.tokens {
		if v.token != WHITESPACE {
			n = append(n, v)
		}
	}

	l.tokens = n
}

func (l *Lexer) advanceLine() {
	l.span.column = 0
	l.span.line++
}

func Lex(reader io.Reader) *Lexer {
	return &Lexer{
		span:   Span{line: 1, column: 0},
		reader: bufio.NewReader(reader),
	}
}
