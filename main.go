package main

import (
	"context"
	"fmt"
	"math/rand/v2"
	"os"
	"strings"
	"time"

	picogo "github.com/picodata/picodata-go"
	logger "github.com/picodata/picodata-go/logger"
	strats "github.com/picodata/picodata-go/strategies"

	"github.com/google/uuid"
)

func main() {
	// export PICODATA_CONNECTION_URL=postgres://username:password@host:port
	ctx := context.Background()
	connectionUrl := os.Getenv("PICODATA_CONNECTION_URL")
	strategy := picogo.WithBalanceStrategy(strats.NewRoundRobinStrategy())
	loggerConfig := picogo.WithLogLevel(logger.LevelError)
	pool, err := picogo.New(ctx, connectionUrl, strategy, loggerConfig)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	defer pool.Close()

	query := "INSERT INTO profiles (id, region, ids, expire) VALUES($1, $2, $3, $4)"
	for i := range(100) {
		profileId := generateId()
		region := randomInRange(1, 120) 
		ids := randomInRangeCount(1, 1000, 10)
		expire := time.Now().Add(time.Duration(100 * time.Second))
		result, err := pool.Exec(ctx, query, profileId, region, ids, expire)
		fmt.Println(i, result, err)
	}
}

func generateId() string {
	result, _ := uuid.NewV6()
	return result.String()
}

func randomInRange(min, max int) string {
	rnd := rand.IntN(max-min) + min
	return fmt.Sprint(rnd)
}

func randomInRangeCount(min, max, cnt int) string {
	result := []string{}
	for range(cnt) {
		rnd := randomInRange(min, max)
		result = append(result, rnd)
	}
	return strings.Join(result, ",") 
}

//func select() {
//	var (
//		id    int
//		name  string
//		stock int
//	)
//	query := "select * from items where id=$1"
//	err = pool.QueryRow(ctx, query, 2).Scan(&id, &name, &stock)
//	if err != nil {
//		fmt.Fprintf(os.Stderr, "QueryRow failed: %v\n", err)
//		os.Exit(1)
//	}
//	fmt.Println(id, name, stock)
//}
