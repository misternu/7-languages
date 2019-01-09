(ns prime-factors.core)

(defn primes
  ([num] (primes num 2))
  ([num c]
    (cond
      (< num c) []
      (zero? (rem num c)) (cons c (primes (/ num c)))
      :else (primes num (inc c)))))
