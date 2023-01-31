package race

import "github.com/alexanderjoseph/formula1/formulagraphql/models/circuits"

type Resp struct {
	MRData MRData `json:"MRData"`
}
type Timings struct {
	DriverID string `json:"driverId"`
	Position string `json:"position"`
	Time     string `json:"time"`
}
type Laps struct {
	Number  string    `json:"number"`
	Timings []Timings `json:"Timings"`
}
type Race struct {
	Season   string            `json:"season"`
	Round    string            `json:"round"`
	URL      string            `json:"url"`
	RaceName string            `json:"raceName"`
	Circuit  circuits.Circuits `json:"Circuit"`
	Date     string            `json:"date"`
	Time     string            `json:"time"`
	Laps     []Laps            `json:"Laps"`
}
type RaceTable struct {
	Season string `json:"season"`
	Races  []Race `json:"Races"`
}
type MRData struct {
	Xmlns     string    `json:"xmlns"`
	Series    string    `json:"series"`
	URL       string    `json:"url"`
	Limit     string    `json:"limit"`
	Offset    string    `json:"offset"`
	Total     string    `json:"total"`
	RaceTable RaceTable `json:"RaceTable"`
}
