package graph

import (
	"encoding/json"
	"net/http"

	"github.com/alexanderjoseph/formula1/formulagraphql/graph/model"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/circuits"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/constructors"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/drivers"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/race"
)

func getTeams(in []constructors.ConstructorStandings, limit int) []*model.TeamStanding {
	ret := []*model.TeamStanding{}
	for i := range in {
		if i == limit {
			break
		}
		team := in[i]
		ret = append(ret, &model.TeamStanding{
			Position: &team.Position,
			Points:   &team.Points,
			Wins:     &team.Wins,
			Team: &model.Constructor{
				ID:          &team.Constructor.ConstructorID,
				Name:        &team.Constructor.Name,
				URL:         &team.Constructor.URL,
				Nationality: &team.Constructor.Nationality,
			},
		})
	}
	return ret
}

func getDrivers(in []drivers.DriverStandings, limit int) []*model.DriverStanding {
	ret := []*model.DriverStanding{}
	for i := range in {
		if i == limit {
			break
		}
		driver := in[i]
		ret = append(ret, &model.DriverStanding{
			Position: &driver.Position,
			Points:   &driver.Points,
			Wins:     &driver.Wins,
			Driver: &model.Driver{
				ID:          &driver.Driver.DriverID,
				URL:         &driver.Driver.URL,
				Nationality: &driver.Driver.Nationality,
				Number:      &driver.Driver.PermanentNumber,
				Code:        &driver.Driver.Code,
				GivenName:   &driver.Driver.GivenName,
				FamilyName:  &driver.Driver.FamilyName,
				DateOfBirth: &driver.Driver.DateOfBirth,
			},
		})
	}
	return ret
}

func getCircuits(in []circuits.Circuits) []*model.Circuit {
	ret := []*model.Circuit{}
	for i := range in {
		circuit := in[i]
		ret = append(ret, getCircuit(circuit))
	}
	return ret
}

func getCircuit(circuit circuits.Circuits) *model.Circuit {
	req, err := http.NewRequest("GET", "http://en.wikipedia.org/w/api.php", nil)
	if err != nil {
		panic(err) // do error handling properly
	}
	// http://en.wikipedia.org/w/api.php?action=query&prop=pageimages&format=json&piprop=original&titles=
	q := req.URL.Query()
	q.Add("action", "query")
	q.Add("prop", "pageimages")
	q.Add("format", "json")
	q.Add("piprop", "original")
	q.Add("titles", circuit.CircuitName)
	req.URL.RawQuery = q.Encode()
	resp, err := http.Get(req.URL.String())
	if err != nil {
		panic(err) // do error handling properly
	}
	defer resp.Body.Close()

	// {"batchcomplete":"","query":{"normalized":[{"from":"Nathalis_iole","to":"Nathalis iole"}],"pages":{"20209834":{"pageid":20209834,"ns":0,"title":"Nathalis iole","original":{"source":"https://upload.wikimedia.org/wikipedia/commons/2/2f/Dainty_Sulphur_Megan_McCarty16.jpg","width":1237,"height":733}}}}}
	var imgResp struct {
		Query struct {
			Pages map[string]struct {
				Original struct {
					Source string `json:"source"`
				} `json:"original"`
			} `json:"pages"`
		} `json:"query"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&imgResp); err != nil {
		panic(err) // do error handling properly
	}

	imgPages := imgResp.Query.Pages
	var imgURL string
	for _, imgPage := range imgPages {
		imgURL = imgPage.Original.Source
		break // there's only one page in here
	}

	return &model.Circuit{
		ID:          &circuit.CircuitID,
		URL:         &circuit.URL,
		CircuitName: &circuit.CircuitName,
		Img:         &imgURL,
		Location: &model.Location{
			Lat:      &circuit.Location.Lat,
			Long:     &circuit.Location.Long,
			Locality: &circuit.Location.Locality,
			Country:  &circuit.Location.Country,
		},
	}
}

func getRaces(in []race.Race) []*model.Race {
	ret := []*model.Race{}
	for i := range in {
		race := in[i]
		// get img from wikipedia
		ret = append(ret, &model.Race{
			Round:    &race.Round,
			URL:      &race.URL,
			RaceName: &race.RaceName,
			Date:     &race.Date,
			Time:     &race.Time,
			Circuit:  getCircuit(race.Circuit),
		})
	}
	return ret
}

func getLapTimes(in []race.Race) []*model.Lap {
	ret := []*model.Lap{}
	for i := range in {
		race := in[i]
		ret = append(ret, &model.Lap{
			Round:    &race.Round,
			URL:      &race.URL,
			RaceName: &race.RaceName,
			Date:     &race.Date,
			Time:     &race.Time,
			Circuit:  getCircuit(race.Circuit),
			Timings:  getTimings(race.Laps[0]), // should check if len > 0
			Lap:      &race.Laps[0].Number,     // should check if len > 0
		})
	}
	return ret
}

func getTimings(in race.Laps) []*model.Timing {
	ret := []*model.Timing{}
	for i := range in.Timings {
		timing := in.Timings[i]
		ret = append(ret, &model.Timing{
			DriverID: &timing.DriverID,
			Position: &timing.Position,
			Time:     &timing.Time,
		})
	}
	return ret
}
