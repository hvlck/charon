package main

type OPCODE uint8

const (
	OP_RETURN = iota
	OP_CONSTANT
)

var ops = []string{
	OP_RETURN:   "OP_RETURN",
	OP_CONSTANT: "OP_CONSTANT",
}

func (t OPCODE) String() string {
	return ops[t]
}

type Chunk struct {
	// total allocation of the chunk array
	capacity int
	// current allocation amount
	size int
	// bytecode
	codes []uint8
	// array of Values/values
	constants ValueArray
}

func (c *Chunk) Write(code uint8) {
	// there's space to store a new code
	if c.size <= c.capacity {
		c.size++
		c.codes[c.size] = code
	} else {
		c.capacity = c.capacity * 2
		n := allocate(c.capacity)
		c.codes = move(c.codes, n)
	}
}

func (c *Chunk) WriteConstant(v Value) {

}

func NewChunk() *Chunk {
	return &Chunk{
		capacity:  8,
		size:      0,
		codes:     make([]uint8, 8),
		constants: *InitialiseValueArray(),
	}
}

// helper functions for increasing chunk allocations

func move(o []uint8, n []uint8) []uint8 {
	return append(n, o[:]...)
}

func allocate(size int) []uint8 {
	return make([]uint8, size)
}
