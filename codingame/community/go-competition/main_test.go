package main

import (
	"bufio"
	"fmt"
	"os"
	"testing"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(make([]byte, 1000000), 1000000)

	var lines int
	scanner.Scan()
	_, _ = fmt.Sscan(scanner.Text(), &lines)

	board := make([]string, lines)
	for i := 0; i < lines; i++ {
		scanner.Scan()
		board[i] = scanner.Text()
	}

	black, white := process(board)

	fmt.Printf("BLACK : %v\n", black)
	fmt.Printf("WHITE : %v.5\n", white)
	var winner string
	if white >= black {
		winner = "WHITE"
	} else {
		winner = "BLACK"
	}
	fmt.Printf("%v WINS", winner)
}

func newEmptyBoard(board []string) [][]bool {
	visited := make([][]bool, len(board))
	for i := range visited {
		visited[i] = make([]bool, len(board[i]))
	}
	return visited
}

func process(board []string) (uint, uint) {
	var black uint = 0
	var white uint = 6
	visitedCells := newEmptyBoard(board)

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if visitedCells[i][j] {
				continue
			}
			cell := rune(board[i][j])
			if cell == 'B' {
				black++
				continue
			} else if cell == 'W' {
				white++
				continue
			}

			visitedRegion := newEmptyBoard(board)
			blackBorder, whiteBoarder, cells := flood(board, visitedCells, visitedRegion, i, j)
			if blackBorder == whiteBoarder {
				continue
			}
			if blackBorder > whiteBoarder {
				black += cells
			} else {
				white += cells
			}

			//TODO debug
			/*for i := 0; i < len(visitedRegion); i++ {
				for j := 0; j < len(visitedRegion[i]); j++ {
					if visitedRegion[i][j] {
						if board[i][j] == '.' {
							if blackBorder > whiteBoarder {
								board[i] = board[i][0:j] + "b" + board[i][j+1:]
							} else {
								board[i] = board[i][0:j] + "w" + board[i][j+1:]
							}
						}
					}
				}
			}*/
			//TODO debug
		}
	}

	//TODO debug
	/*for i := 0; i < len(board); i++ {
		println(board[i])
	}*/
	//TODO debug

	return black, white
}

func flood(board []string, visitedCells [][]bool, visitedRegion [][]bool, i int, j int) (uint, uint, uint) {
	if i < 0 || j < 0 || i >= len(board) || j >= len(board[i]) || // Out of bounds
		visitedRegion[i][j] {
		return 0, 0, 0
	}

	cell := rune(board[i][j])
	visitedRegion[i][j] = true

	var blackBorder uint = 0
	var whiteBorder uint = 0
	var cells uint = 0
	if cell == 'B' {
		blackBorder++
	} else if cell == 'W' {
		whiteBorder++
	} else {
		visitedCells[i][j] = true
		cells++
		abb, awb, ac := flood(board, visitedCells, visitedRegion, i, j+1)
		blackBorder += abb
		whiteBorder += awb
		cells += ac
		abb, awb, ac = flood(board, visitedCells, visitedRegion, i, j-1)
		blackBorder += abb
		whiteBorder += awb
		cells += ac
		abb, awb, ac = flood(board, visitedCells, visitedRegion, i+1, j)
		blackBorder += abb
		whiteBorder += awb
		cells += ac
		abb, awb, ac = flood(board, visitedCells, visitedRegion, i-1, j)
		blackBorder += abb
		whiteBorder += awb
		cells += ac
	}

	return blackBorder, whiteBorder, cells
}

// https://www.codingame.com/training/medium/go-competition

func Test_process(t *testing.T) {
	tests := []struct {
		name  string
		board []string
		black uint
		white uint
	}{
		{
			"basic1",
			[]string{
				"....BW...",
				"....BW...",
				"....BW...",
				"....BW...",
				"...BBW...",
				"...BWW...",
				"...BW....",
				"...BW....",
				"...BW....",
			},
			41, 46,
		},
		{
			"hard1",
			[]string{
				".W.WW...WWWB...B.W.",
				"WWWBWW..WWBB..BBWW.",
				"BBBBBW.W.WWB.BBWW..",
				"...BWW.W..WWBBW.WW.",
				"...BBBW...WWBWW.W..",
				"....BW.WWWBWW..W...",
				"...BBWWWWBBBWWWW...",
				"..BBWBBWBB.BW...WW.",
				".BWWWWBBBBBWWW...WW",
				"..BBW.WB.BBBBW.....",
				"..BW.W.WBBWWW......",
				"..BWW..WBWWBWW.....",
				"...BW.WWWBBBBWW....",
				"...BW.WBWB.BWWWW...",
				"..BWWWBBB.BBWBBWWW.",
				"..BB.WW.BBBWWBBBBWW",
				"...BBBWWB..BWWBWBBW",
				"....BWWBB..BW.WWB.B",
				"...BWWWWB..BBW.WBB.",
			},
			161, 203,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			black, white := process(tt.board)
			if black != tt.black || white != tt.white {
				t.Errorf("%v got = (%v,%v.5), want = (%v,%v.5)", tt.name, black, white, tt.black, tt.white)
			}
		})
	}
}
