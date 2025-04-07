package main

import "fmt"

func main() {
	limit := 400000
	a,b := 1,1
	total := 0
	for true{
		c := a + b
		if c > limit {
			break
		}
		if c % 2 ==0 {
			total +=c
		}
		a = b
		b = c
	}
	fmt.Println(total)
}
