package main

import (
	"io/ioutil"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type layerColorCounter []map[int]int

func (lcc layerColorCounter) Len() int {
	return len(lcc)
}

func (lcc layerColorCounter) Swap(i, j int) {
	lcc[i], lcc[j] = lcc[j], lcc[i]
}

func (lcc layerColorCounter) Less(i, j int) bool {
	return lcc[i][0] < lcc[j][0]
}

type layer struct {
	width  int
	height int
	data   []int
}

func (l *layer) countColors() map[int]int {
	counter := make(map[int]int)

	for y := 0; y < l.height; y++ {
		for x := 0; x < l.width; x++ {
			pixelValue := l.data[y*l.width+x]
			counter[pixelValue] += 1
		}
	}
	return counter
}

type image struct {
	width  int
	height int
	layers []layer
}

func (i *image) layerCount() int {
	return len(i.layers)
}

func (i *image) getLayer(index int) *layer {
	return &i.layers[index]
}

func (img *image) getLayeredPixel(x int, y int) string {
	for _, layer := range img.layers {
		pixel := layer.data[y*img.width+x]
		if pixel == 0 {
			return " "
		} else if pixel == 1 {
			return "X"
		}
	}

	return "."
}

func (img *image) render() {
	for y := 0; y < img.height; y++ {
		row := make([]string, img.width)

		for x := 0; x < img.width; x++ {
			row[x] = img.getLayeredPixel(x, y)
		}

		log.Println(strings.Join(row, ""))
	}
}

func loadImage(input string, width int, height int) *image {
	image := &image{width: width, height: height}
	layerSize := width * height

	if len(input)%layerSize != 0 {
		log.Fatalf("Incorrect size of input: %d", len(input))
	}

	numLayers := len(input) / layerSize
	image.layers = make([]layer, numLayers)

	for layerIndex := 0; layerIndex < numLayers; layerIndex++ {
		currentLayer := layer{width: width, height: height, data: make([]int, layerSize)}

		for y := 0; y < height; y++ {
			for x := 0; x < width; x++ {
				pixel, err := strconv.Atoi(string([]byte{input[layerIndex*layerSize+(y*width)+x]}))
				if err != nil {
					log.Fatal("Incorrect input value")
				}

				currentLayer.data[y*width+x] = pixel
			}
		}

		image.layers[layerIndex] = currentLayer
	}

	return image
}

func (i *image) countColorsByLayer() layerColorCounter {
	counter := make(layerColorCounter, len(i.layers))

	for index, layer := range i.layers {
		counter[index] = layer.countColors()
	}

	return counter
}

func problem1(input string) {
	image := loadImage(input, 25, 6)
	counter := image.countColorsByLayer()
	sort.Sort(counter)
	log.Printf("Answer: %d", counter[0][1]*counter[0][2])
}

func problem2(input string) {
	image := loadImage(input, 25, 6)
	image.render()
}

func main() {
	bytes_input, err := ioutil.ReadFile("../../inputs/day08.txt")
	if err != nil {
		log.Fatal(err)
	}

	input := string(bytes_input[:len(bytes_input)-1])

	if os.Args[1] == "problem1" {
		problem1(input)
	} else if os.Args[1] == "problem2" {
		problem2(input)
	} else {
		log.Fatal("Must provide one of: [problem1, problem2]")
	}
}
