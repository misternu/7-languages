package prime_factors

func Factors(num int) []int {
  factors := []int{}
  for c := 2; c <= num; c++ {
    for ; num % c == 0; num /= c {
      factors = append(factors, c)
    }
  }
  return factors
}
