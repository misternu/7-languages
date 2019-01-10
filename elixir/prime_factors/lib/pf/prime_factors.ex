defmodule PF.PrimeFactors do
  def prime_factors(num, c \\ 2) do
    cond do
      num == 1 -> []
      rem(num, c) == 0 ->
        [c] ++ prime_factors(div(num, c), c)
      true -> prime_factors(num, c + 1)
    end
  end
end
