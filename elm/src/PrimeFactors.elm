module PrimeFactors exposing (..)

primeFactors : Int -> (List Int)
primeFactors x = primeFactorsPrime x 2

primeFactorsPrime : Int -> Int -> (List Int)
primeFactorsPrime x c =
  if x < 2 then []
  else if remainderBy c x == 0 then
    [c] ++ primeFactorsPrime (x // c) c
  else primeFactorsPrime x (c + 1)
