module Main (main) where

main :: IO ()
main = do
  let x = doubleMe 4
  putStrLn "gfello world"

doubleMe :: Int -> Int
doubleMe x = x + x
