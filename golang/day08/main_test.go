package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestCountColorsByLayer(t *testing.T) {
	input := "123456789012"
	image := loadImage(input, 3, 2)
	counter := image.countColorsByLayer()

	assert.Equal(t, counter[0][1], 1)
	assert.Equal(t, counter[0][2], 1)
	assert.Equal(t, counter[0][3], 1)
	assert.Equal(t, counter[0][4], 1)
	assert.Equal(t, counter[0][5], 1)
	assert.Equal(t, counter[0][6], 1)
	assert.Equal(t, counter[0][7], 0)
	assert.Equal(t, counter[0][8], 0)
	assert.Equal(t, counter[0][9], 0)
	assert.Equal(t, counter[0][0], 0)

	assert.Equal(t, counter[1][1], 1)
	assert.Equal(t, counter[1][2], 1)
	assert.Equal(t, counter[1][3], 0)
	assert.Equal(t, counter[1][4], 0)
	assert.Equal(t, counter[1][5], 0)
	assert.Equal(t, counter[1][6], 0)
	assert.Equal(t, counter[1][7], 1)
	assert.Equal(t, counter[1][8], 1)
	assert.Equal(t, counter[1][9], 1)
	assert.Equal(t, counter[1][0], 1)
}
