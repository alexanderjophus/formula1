package raceresults

type Resp struct {
	MRData MRData `json:"MRData"`
}
type Location struct {
	Lat      string `json:"lat"`
	Long     string `json:"long"`
	Locality string `json:"locality"`
	Country  string `json:"country"`
}
type Circuit struct {
	CircuitID   string   `json:"circuitId"`
	URL         string   `json:"url"`
	CircuitName string   `json:"circuitName"`
	Location    Location `json:"Location"`
}
type Driver struct {
	DriverID        string `json:"driverId"`
	PermanentNumber string `json:"permanentNumber"`
	Code            string `json:"code"`
	URL             string `json:"url"`
	GivenName       string `json:"givenName"`
	FamilyName      string `json:"familyName"`
	DateOfBirth     string `json:"dateOfBirth"`
	Nationality     string `json:"nationality"`
}
type Constructor struct {
	ConstructorID string `json:"constructorId"`
	URL           string `json:"url"`
	Name          string `json:"name"`
	Nationality   string `json:"nationality"`
}
type Time struct {
	Millis string `json:"millis"`
	Time   string `json:"time"`
}
type AverageSpeed struct {
	Units string `json:"units"`
	Speed string `json:"speed"`
}
type FastestLap struct {
	Rank         string       `json:"rank"`
	Lap          string       `json:"lap"`
	Time         Time         `json:"Time"`
	AverageSpeed AverageSpeed `json:"AverageSpeed"`
}
type Results struct {
	Number       string      `json:"number"`
	Position     string      `json:"position"`
	PositionText string      `json:"positionText"`
	Points       string      `json:"points"`
	Driver       Driver      `json:"Driver"`
	Constructor  Constructor `json:"Constructor"`
	Grid         string      `json:"grid"`
	Laps         string      `json:"laps"`
	Status       string      `json:"status"`
	Time         Time        `json:"Time,omitempty"`
	FastestLap   FastestLap  `json:"FastestLap"`
}
type Races struct {
	Season   string    `json:"season"`
	Round    string    `json:"round"`
	URL      string    `json:"url"`
	RaceName string    `json:"raceName"`
	Circuit  Circuit   `json:"Circuit"`
	Date     string    `json:"date"`
	Time     string    `json:"time"`
	Results  []Results `json:"Results"`
}
type RaceTable struct {
	Season string  `json:"season"`
	Round  string  `json:"round"`
	Races  []Races `json:"Races"`
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
