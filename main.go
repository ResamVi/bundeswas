package main

import (
	"context"
	"fmt"
	"os"

	"github.com/resamvi/bundeswas/dip"
)

const (
	url               = "https://search.dip.bundestag.de/api/v1"
	plenarprotokollID = 908
)

var (
	apikey = "rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF"
)

// Example: https://search.dip.bundestag.de/api/v1/plenarprotokoll/908?apikey=rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF
//
//go:generate go run github.com/ogen-go/ogen/cmd/ogen@latest --target dip --package dip --clean --config ogen_config.yaml openapi.yaml
func main() {
	if value, ok := os.LookupEnv("API_KEY"); ok {
		apikey = value
	}

	auth := Auth{}
	client, err := dip.NewClient(url, auth)
	if err != nil {
		panic(err)
	}

	params := dip.GetPlenarprotokollParams{
		ID:     plenarprotokollID,
		Format: dip.NewOptGetPlenarprotokollFormat(dip.GetPlenarprotokollFormatJSON),
	}

	plenarprotokoll, err := client.GetPlenarprotokoll(context.Background(), params)
	if err != nil {
		panic(err)
	}

	p := plenarprotokoll.(*dip.Plenarprotokoll)

	fmt.Println(p.Herausgeber)
}

type Auth struct{}

func (a Auth) ApiKeyHeader(ctx context.Context, operationName string) (dip.ApiKeyHeader, error) {
	return dip.ApiKeyHeader{APIKey: apikey}, nil
}

func (a Auth) ApiKeyQuery(ctx context.Context, operationName string) (dip.ApiKeyQuery, error) {
	return dip.ApiKeyQuery{APIKey: apikey}, nil
}
