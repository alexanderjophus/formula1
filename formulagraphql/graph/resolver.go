package graph

import (
	"net/http"
	"time"
)

type Resolver struct {
	client  *http.Client
	baseURL string
}

func NewResolver() *Resolver {
	return &Resolver{
		client: &http.Client{
			Timeout: 10 * time.Second,
		},
		baseURL: "http://ergast.com/api/f1",
	}
}
