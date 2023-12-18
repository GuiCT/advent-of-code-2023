import Data.Char (isDigit)
import Debug.Trace
import System.IO

main :: IO ()
main = do
  content <- readFile "input.txt"
  let linesOfFile = lines content
  let sumOfPowers = sum $ map processLine linesOfFile
  print sumOfPowers

-- https://stackoverflow.com/questions/4978578/how-to-split-a-string-in-haskell
-- Split string by char.
-- Takes a predicate (Char -> Bool) and a String, returns a list of String
wordsWhen :: (Char -> Bool) -> String -> [String]
wordsWhen p s = case dropWhile p s of
  -- Empty string: empty list
  "" -> []
  -- Non-empty string: recursively apply the function to the tail
  s' -> w : wordsWhen p s''
    where
      (w, s'') = break p s'

processLine :: String -> Int
processLine line = if count > 0 then 2 ^ (count - 1) else 0
  where
    count = length $ filter (`elem` lhsNumbers) rhsNumbers
    lhsNumbers = map (read :: String -> Int) $ wordsWhen (== ' ') lhs
    rhsNumbers = map (read :: String -> Int) $ wordsWhen (== ' ') rhs
    (lhs : rhs : _) = wordsWhen (== '|') afterTwoDots
    (_ : afterTwoDots : _) = wordsWhen (== ':') line
