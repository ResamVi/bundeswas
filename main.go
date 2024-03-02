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

// oapi-codegen -package dip -generate types,client -include-tags Plenarprotokolle openapi.yaml > dip/dip.gen.go
//
// Example: https://search.dip.bundestag.de/api/v1/plenarprotokoll/908?apikey=rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF
func main() {
	if value, ok := os.LookupEnv("API_KEY"); ok {
		apikey = value
	}

	client, err := dip.NewClientWithResponses(url, dip.Authenticate(apikey))
	if err != nil {
		panic(err)
	}

	params := &dip.GetPlenarprotokollParams{
		Format: ref(dip.Json),
	}

	plenarprotokoll, err := client.GetPlenarprotokollWithResponse(context.Background(), plenarprotokollID, params)
	if err != nil {
		panic(err)
	}

	fmt.Println(plenarprotokoll.JSON200)
}

func ref[T any](obj T) *T {
	return &obj
}
