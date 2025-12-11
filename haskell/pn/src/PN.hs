module PN (pnSucc, pnShow, pnNumbers, toDecimal) where

pnNumbers :: [[Integer]]
pnNumbers = [0] : [1] : [ pnSucc a | a <- tail pnNumbers ]

pnSucc :: [Integer] -> [Integer]
pnSucc xs =
  reverse $ pnSuccR (reverse xs)

pnSuccR :: [Integer] -> [Integer]
pnSuccR xs = 
  let 
    baseInt = foldr (\_ -> (+) 1) 0 xs
    b = toInteger baseInt
    (rollOvers, digits) = break (< b) xs
    len = foldr(\_ -> (+) 1) 0 rollOvers
  in
    case digits of
      [] -> (b + 1) : replicate baseInt 0
      h:ds | h == (b - 1) || len == (b - 1) -> map (const 0) rollOvers ++ [h + 1] ++ ds
      h:ds -> rollOvers ++ [h + 1] ++ ds

---- 

toDecimal :: [Integer] -> Integer
toDecimal pn =
  foldl (\acc (n,p) -> acc * (p ^ n)) 1 withPrimes
  where
    withPrimes = zip (reverse pn) primes

primes :: [Integer]
primes = sieve [2..]
  where
    sieve [] = [2]
    sieve (p:xs) = p : sieve [ x | x <- xs, mod x p > 0 ]

pnShow :: [Integer] -> [Char]
pnShow pn =
  show pn ++ " : " ++ show (toDecimal pn)

