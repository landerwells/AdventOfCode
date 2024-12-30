import System.IO
import Control.Exception

main :: IO ()
main = do
  contents <- readFile "../../inputs/2017/1"
  putStrLn contents 



sumAdjacent :: String -> Int

