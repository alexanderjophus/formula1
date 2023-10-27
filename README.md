# Formula GraphQL

Experimental play with GraphQL - needs major refactor

Goals:
- Transform the Formula 1 ergast API into a graphQL API

## Running

### Backend

```sh
go run formulagraphql/main.go
```

Open a browser to localhost:8080.

Try a payload of 

```graphql
query {
  DriverStandings(filter: {year: "2018", top: 5}){
    drivers {
      points
      Driver {
        code
        givenName
        familyName
      }
    }
  }
}
```

### Frontend

```sh
cd web
yarn install
yarn serve
```

Open a browser to localhost:8081.

### Tests

```sh
go test ./...
```

## Hacking

Modify the `graph/schema.graphqls` file, and run `go run github.com/99designs/gqlgen generate`.
This should update any resolver function signatures, then modify the resolver function itself to support the new updates.

## Notes

This was being actively developed on twitch (it may come back) :)