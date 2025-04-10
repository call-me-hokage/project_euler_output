package main

import "fmt"

func main() {
	number := 600851475143
	factor := 2

	for factor*factor <= number {
		if number%factor == 0 {
			number /= factor // Divide number by factor
		} else {
			factor += 1 // Move to the next factor
		}
	}

	fmt.Println("The largest prime factor is:", number)
}
