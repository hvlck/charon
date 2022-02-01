package main

type Value float64

type ValueArray struct {
	// total allocation of the chunk array
	capacity int
	// current allocation amount
	size int
	// value
	values []Value
}

func (v []uint8) Value() []Value {
	n := make([]Value, len(v))

	for _, t := range v {
		n = append(n, float64(t))
		t.(float64)
	}
}

func (c *ValueArray) Write(code uint8) {
	// there's space to store a new code
	if c.size <= c.capacity {
		c.size++
		c.values[c.size] = code
	} else {
		c.capacity = c.capacity * 2
		n := allocate(c.capacity)
		c.values = move_value(c.values, Value(n))
	}
}

func InitialiseValueArray() *ValueArray {
	return &ValueArray{
		capacity: 8,
		size:     0,
		values:   0,
	}
}

func move_value(o []Value, n []Value) []Value {
	var v []Value
	v = append(n, o[:]...)
	return v
}
