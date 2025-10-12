package main

import (
	"context"
	"fmt"
	"os"

	picogo "github.com/picodata/picodata-go"
	logger "github.com/picodata/picodata-go/logger"
	strats "github.com/picodata/picodata-go/strategies"
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
	/*
	   CREATE TABLE items (id INTEGER NOT NULL,name TEXT NOT NULL,stock INTEGER,PRIMARY KEY (id)) USING memtx DISTRIBUTED BY (id) OPTION (TIMEOUT = 3.0);
	   INSERT INTO items VALUES
	   (1, 'bricks', 1123),
	   (2, 'panels', 998),
	   (3, 'piles', 177);
	*/
	var (
		id    int
		name  string
		stock int
	)
	query := "select * from items where id=$1"
	err = pool.QueryRow(ctx, query, 2).Scan(&id, &name, &stock)
	if err != nil {
		fmt.Fprintf(os.Stderr, "QueryRow failed: %v\n", err)
		os.Exit(1)
	}

	fmt.Println(id, name, stock)
}
