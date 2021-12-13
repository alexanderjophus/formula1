package constructors

type ConstructorsResp struct {
	MRData MRData `json:"MRData"`
}
type Constructor struct {
	ConstructorID string `json:"constructorId"`
	URL           string `json:"url"`
	Name          string `json:"name"`
	Nationality   string `json:"nationality"`
}
type ConstructorStandings struct {
	Position     string      `json:"position"`
	PositionText string      `json:"positionText"`
	Points       string      `json:"points"`
	Wins         string      `json:"wins"`
	Constructor  Constructor `json:"Constructor"`
}
type StandingsLists struct {
	Season               string                 `json:"season"`
	Round                string                 `json:"round"`
	ConstructorStandings []ConstructorStandings `json:"ConstructorStandings"`
}
type StandingsTable struct {
	Season         string           `json:"season"`
	StandingsLists []StandingsLists `json:"StandingsLists"`
}
type MRData struct {
	Xmlns          string         `json:"xmlns"`
	Series         string         `json:"series"`
	URL            string         `json:"url"`
	Limit          string         `json:"limit"`
	Offset         string         `json:"offset"`
	Total          string         `json:"total"`
	StandingsTable StandingsTable `json:"StandingsTable"`
}
