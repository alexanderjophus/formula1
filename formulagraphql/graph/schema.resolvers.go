package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"

	"github.com/alexanderjoseph/formula1/formulagraphql/graph/generated"
	"github.com/alexanderjoseph/formula1/formulagraphql/graph/model"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/circuits"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/constructors"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/drivers"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/race"
	"github.com/alexanderjoseph/formula1/formulagraphql/models/raceresults"
)

func (r *queryResolver) ConstructorStandings(ctx context.Context, filter *model.StandingsFilter) (*model.ConstructorStandingsReport, error) {
	round := ""
	if filter != nil && filter.Round != nil {
		round = fmt.Sprintf("/%s", *filter.Round)
	}
	resp, err := r.client.Get(fmt.Sprintf("%s/%s%s/constructorStandings.json", r.baseURL, *filter.Year, round))
	if err != nil {
		return nil, fmt.Errorf("getting constructor standings from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var cr constructors.ConstructorsResp
	err = json.NewDecoder(resp.Body).Decode(&cr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(cr.MRData.StandingsTable.StandingsLists) == 0 {
		return nil, fmt.Errorf("standings not found")
	}

	standings := cr.MRData.StandingsTable.StandingsLists[0]

	teams := getTeams(standings.ConstructorStandings, *filter.Top)

	ret := &model.ConstructorStandingsReport{
		Season: &standings.Season,
		Round:  &standings.Round,
		Teams:  teams,
	}

	return ret, nil
}

func (r *queryResolver) DriverStandings(ctx context.Context, filter *model.StandingsFilter) (*model.DriverStandingsReport, error) {
	round := ""
	if filter != nil && filter.Round != nil {
		round = fmt.Sprintf("/%s", *filter.Round)
	}
	resp, err := r.client.Get(fmt.Sprintf("%s/%s%s/driverStandings.json", r.baseURL, *filter.Year, round))
	if err != nil {
		return nil, fmt.Errorf("getting driver standings from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var dr drivers.DriversResp
	err = json.NewDecoder(resp.Body).Decode(&dr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(dr.MRData.StandingsTable.StandingsLists) == 0 {
		return nil, fmt.Errorf("standings not found")
	}

	standings := dr.MRData.StandingsTable.StandingsLists[0]

	drivers := getDrivers(standings.DriverStandings, *filter.Top)

	ret := &model.DriverStandingsReport{
		Season:  &standings.Season,
		Round:   &standings.Round,
		Drivers: drivers,
	}

	return ret, nil
}

func (r *queryResolver) DriversSeasonalRecords(ctx context.Context, filter *model.StandingsFilter) (*model.DriverGraphReport, error) {
	// get the schedule for the season
	resp, err := r.client.Get(fmt.Sprintf("%s/%s.json", r.baseURL, *filter.Year))
	if err != nil {
		return nil, fmt.Errorf("getting schedule from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var cr race.Resp
	err = json.NewDecoder(resp.Body).Decode(&cr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(cr.MRData.RaceTable.Races) == 0 {
		return nil, fmt.Errorf("schedule not found")
	}

	drivers := make(map[string]*model.DriverGraph)

	// iterate through ever race of a season and get the results for each driver
	for _, race := range cr.MRData.RaceTable.Races {
		resp, err := r.client.Get(fmt.Sprintf("%s/%s/%s/results.json", r.baseURL, *filter.Year, race.Round))
		if err != nil {
			return nil, fmt.Errorf("getting driver standings from ergast: %w", err)
		}

		if resp.StatusCode != http.StatusOK {
			return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
		}

		var rr raceresults.Resp
		err = json.NewDecoder(resp.Body).Decode(&rr)
		if err != nil {
			return nil, fmt.Errorf("decoding response: %w", err)
		}

		if len(rr.MRData.RaceTable.Races) != 0 {
			wg := sync.WaitGroup{}
			for _, result := range rr.MRData.RaceTable.Races[0].Results {
				wg.Add(1)
				func(round string, result raceresults.Results) {
					defer wg.Done()
					if drivers[result.Driver.DriverID] == nil {
						drivers[result.Driver.DriverID] = &model.DriverGraph{
							Records: []*model.Record{},
						}
					}
					drivers[result.Driver.DriverID].Records = append(drivers[result.Driver.DriverID].Records, &model.Record{
						Round:    &round,
						Points:   &result.Points,
						Position: &result.Position,
					})
					if drivers[result.Driver.DriverID].Driver == nil {
						drivers[result.Driver.DriverID].Driver = &model.Driver{
							ID:          &result.Driver.DriverID,
							Number:      &result.Driver.PermanentNumber,
							Code:        &result.Driver.Code,
							URL:         &result.Driver.URL,
							GivenName:   &result.Driver.GivenName,
							FamilyName:  &result.Driver.FamilyName,
							DateOfBirth: &result.Driver.DateOfBirth,
							Nationality: &result.Driver.Nationality,
						}
					}
				}(race.Round, result)
			}
			wg.Wait()
		}
	}

	retDrivers := make([]*model.DriverGraph, 0)
	for _, driver := range drivers {
		retDrivers = append(retDrivers, driver)
	}

	return &model.DriverGraphReport{
		Season:  filter.Year,
		Drivers: retDrivers,
	}, nil
}

func (r *queryResolver) Circuits(ctx context.Context, year *string) (*model.CircuitsReport, error) {
	resp, err := r.client.Get(fmt.Sprintf("%s/%s/circuits.json", r.baseURL, *year))
	if err != nil {
		return nil, fmt.Errorf("getting circuits from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var cr circuits.CircuitsResp
	err = json.NewDecoder(resp.Body).Decode(&cr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(cr.MRData.CircuitTable.Circuits) == 0 {
		return nil, fmt.Errorf("circuits not found")
	}

	circuits := getCircuits(cr.MRData.CircuitTable.Circuits)

	ret := &model.CircuitsReport{
		Season:   &cr.MRData.CircuitTable.Season,
		Circuits: circuits,
	}

	return ret, nil
}

func (r *queryResolver) Schedule(ctx context.Context, year *string) (*model.ScheduleReport, error) {
	resp, err := r.client.Get(fmt.Sprintf("%s/%s.json", r.baseURL, *year))
	if err != nil {
		return nil, fmt.Errorf("getting schedule from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var cr race.Resp
	err = json.NewDecoder(resp.Body).Decode(&cr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(cr.MRData.RaceTable.Races) == 0 {
		return nil, fmt.Errorf("schedule not found")
	}

	races := getRaces(cr.MRData.RaceTable.Races)

	ret := &model.ScheduleReport{
		Season: &cr.MRData.RaceTable.Season,
		Races:  races,
	}

	return ret, nil
}

func (r *queryResolver) LapTimes(ctx context.Context, filter *model.LapTimesFilter) (*model.LapTimesReport, error) {
	resp, err := r.client.Get(fmt.Sprintf("%s/%s/%s/laps/%s.json", r.baseURL, *filter.Year, *filter.Round, *filter.Lap))
	if err != nil {
		return nil, fmt.Errorf("getting lap times from ergast: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var rr race.Resp
	err = json.NewDecoder(resp.Body).Decode(&rr)
	if err != nil {
		return nil, fmt.Errorf("decoding response: %w", err)
	}

	if len(rr.MRData.RaceTable.Races) == 0 {
		return nil, fmt.Errorf("laps not found")
	}

	laps := getLapTimes(rr.MRData.RaceTable.Races)

	ret := &model.LapTimesReport{
		Season: &rr.MRData.RaceTable.Season,
		Laps:   laps,
	}

	return ret, nil
}

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type queryResolver struct{ *Resolver }
