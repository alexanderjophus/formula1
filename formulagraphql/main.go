package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/alexanderjoseph/formula1/formulagraphql/graph"
	"github.com/alexanderjoseph/formula1/formulagraphql/graph/generated"
	"github.com/go-chi/chi"
	"github.com/rs/cors"
	"go.uber.org/zap"
)

const defaultPort = "8080"

func logging(log *zap.Logger, next http.Handler) http.Handler {
	type bodyGraphQL struct {
		OperationName string `json:"operationName"`
	}

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()

		buf, err := io.ReadAll(r.Body)
		if err != nil {
			log.Warn("reading request body", zap.Error(err))
		}
		b := io.NopCloser(bytes.NewBuffer(buf))
		r.Body = io.NopCloser(bytes.NewBuffer(buf))

		next.ServeHTTP(w, r)

		var body bodyGraphQL
		if err := json.NewDecoder(b).Decode(&body); err != nil {
			log.Warn("decoding request body", zap.Error(err))
		}

		log.Info("request",
			zap.String("duration", time.Since(start).String()),
			zap.String("operationName", body.OperationName),
		)
	})
}

func main() {
	logger, err := zap.NewProduction()
	if err != nil {
		log.Fatalf("new logger: %v", err)
	}
	defer logger.Sync() // flushes buffer, if any

	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	router := chi.NewRouter()

	router.Use(cors.Default().Handler)

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: graph.NewResolver()}))

	router.Handle("/", playground.Handler("GraphQL playground", "/query"))
	router.Handle("/query", logging(logger, srv))

	logger.Info(fmt.Sprintf("connect to http://localhost:%s/ for GraphQL playground", port))
	if err := http.ListenAndServe(":"+port, router); err != nil {
		logger.Fatal(err.Error())
	}
}
