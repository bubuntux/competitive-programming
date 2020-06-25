package main

import (
	"fmt"
	"reflect"
	"strings"
	"testing"
)

func main() {
	var N int
	fmt.Scan(&N)

	players := make([]Player, N)
	for i := 0; i < N; i++ {
		var NUMPLAYER int
		var SIGNPLAYER string
		fmt.Scan(&NUMPLAYER, &SIGNPLAYER)
		players[i] = Player{id: NUMPLAYER, sign: Sign(SIGNPLAYER), opponents: []int{}}
	}

	winner := solve(players)
	fmt.Println(winner.id)
	fmt.Println(strings.Trim(fmt.Sprint(winner.opponents), "[]"))
}

type Sign string

const (
	Rock     Sign = "R"
	Paper    Sign = "P"
	Scissors Sign = "C"
	Lizard   Sign = "L"
	Spock    Sign = "S"
)

type Player struct {
	id        int
	sign      Sign
	opponents []int
}

func (p *Player) addOpponent(opponent int) {
	p.opponents = append(p.opponents, opponent)
}

func solve(players []Player) Player {
	nextRound := make([]Player, len(players)/2)
	for i, j := 0, 0; i < len(players); i, j = i+1, j+1 {
		p1 := &players[i]
		i++
		p2 := &players[i]

		winner := game(p1, p2)
		if winner.id == p1.id {
			p1.addOpponent(p2.id)
		} else {
			p2.addOpponent(p1.id)
		}
		nextRound[j] = *winner
	}
	if len(nextRound) == 1 {
		return nextRound[0]
	}
	return solve(nextRound)
}

func game(p1 *Player, p2 *Player) *Player {
	switch p1.sign {
	case Scissors:
		if p2.sign == Paper || p2.sign == Lizard {
			return p1
		}
		if p2.sign == Spock || p2.sign == Rock {
			return p2
		}
	case Paper:
		if p2.sign == Rock || p2.sign == Spock {
			return p1
		}
		if p2.sign == Scissors || p2.sign == Lizard {
			return p2
		}
	case Rock:
		if p2.sign == Lizard || p2.sign == Scissors {
			return p1
		}
		if p2.sign == Spock || p2.sign == Paper {
			return p2
		}
	case Lizard:
		if p2.sign == Spock || p2.sign == Paper {
			return p1
		}
		if p2.sign == Scissors || p2.sign == Rock {
			return p2
		}
	case Spock:
		if p2.sign == Scissors || p2.sign == Rock {
			return p1
		}
		if p2.sign == Lizard || p2.sign == Paper {
			return p2
		}
	}
	if p1.id < p2.id {
		return p1
	}
	return p2
}

// https://www.codingame.com/ide/puzzle/rock-paper-scissors-lizard-spock

func Test_solve(t *testing.T) {
	tests := []struct {
		name      string
		players   []Player
		winner    int
		opponents []int
	}{
		{"01", []Player{
			{id: 4, sign: "R", opponents: []int{}},
			{id: 1, sign: "P", opponents: []int{}},
			{id: 8, sign: "P", opponents: []int{}},
			{id: 3, sign: "R", opponents: []int{}},
			{id: 7, sign: "C", opponents: []int{}},
			{id: 5, sign: "S", opponents: []int{}},
			{id: 6, sign: "L", opponents: []int{}},
			{id: 2, sign: "L", opponents: []int{}},
		}, 2, []int{6, 5, 1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := solve(tt.players)
			if got.id != tt.winner {
				t.Errorf("%v solve() winner = %v, wanted %v", tt.name, got, tt.winner)
			}
			if !reflect.DeepEqual(got.opponents, tt.opponents) {
				t.Errorf("%v solve() opponents = %v, wanted %v", tt.name, got.opponents, tt.opponents)
			}
		})
	}
}
