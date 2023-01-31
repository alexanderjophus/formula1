package main

import (
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
	"go.uber.org/zap/zapcore"
)

const defaultPort = "8080"

func logging(log *zap.SugaredLogger) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			start := time.Now()

			next.ServeHTTP(w, r)

			log.Info("call", []zapcore.Field{
				zap.String("latency", time.Since(start).String()),
				zap.String("method", r.Method),
			})
		})
	}
}

func main() {
	logger, err := zap.NewProduction()
	if err != nil {
		log.Fatalf("new logger: %v", err)
	}
	defer logger.Sync() // flushes buffer, if any
	sugar := logger.Sugar()

	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	router := chi.NewRouter()

	router.Use(cors.Default().Handler)

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: graph.NewResolver()}))
	loggingMiddleware := logging(sugar)

	router.Handle("/", playground.Handler("GraphQL playground", "/query"))
	router.Handle("/query", loggingMiddleware(srv))

	log.Printf("connect to http://localhost:%s/ for GraphQL playground", port)
	log.Fatal(http.ListenAndServe(":"+port, router))
}
