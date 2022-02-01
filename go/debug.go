package main

import "fmt"

func (c *Chunk) Disassemble(name string) {
	fmt.Printf("\n===== chunk \"%v\" =====\n", name)

	for i := 0; i < c.size; i++ {
		c.DisassembleCode(i)
	}
}

func (c *Chunk) DisassembleCode(idx int) {
	fmt.Printf("%04d ", idx)

	switch c.codes[idx] {
	case OP_RETURN:
		print(OP_RETURN, idx)
	}
}

func print(name OPCODE, idx int) int {
	fmt.Printf("%s\n", name)

	return idx + 1
}
