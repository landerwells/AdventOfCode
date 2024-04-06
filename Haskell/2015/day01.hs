import System.IO
import Control.Exception

main :: IO ()
main = do
  contents <- readFile "../../inputs/2015/1"
  let finalFloor = sum $ map charToFloor contents
  putStrLn $ "Final floor: " ++ show finalFloor


charToFloor :: Char -> Int
charToFloor '(' = 1
charToFloor ')' = -1
