package fetch

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

type Response[T any] struct {
	NumFound  int        `json:"numFound"`
	Documents []T        `json:"documents"`
	Cursor    string     `json:"cursor"`
}

var invalids = strings.NewReplacer("/", "%2F", "+", "%2B")

func All[T any](url string) []T {
	result := make([]T, 0)

	cursor := url
	for {
		resp := fetch[T](cursor)
		if len(resp.Documents) == 0 {
			break
		}

		result = append(result, resp.Documents...)

		cursor = fmt.Sprintf("%s&cursor=%s", url, invalids.Replace(resp.Cursor))
		time.Sleep(2 * time.Second)
	}

	return result
}

func One[T any](url string) T {
	req, err := http.NewRequest(http.MethodGet, url, nil)
	checkErr(err)

	client := http.Client{}
	resp, err := client.Do(req)
	checkErr(err)

	bytes, err := io.ReadAll(resp.Body)
	checkErr(err)

	var result T
	err = json.Unmarshal(bytes, &result)
	checkErr(err)

	return result
}

func fetch[T any](url string) Response[T] {
	req, err := http.NewRequest(http.MethodGet, url, nil)
	checkErr(err)

	client := http.Client{}
	resp, err := client.Do(req)
	checkErr(err)

	bytes, err := io.ReadAll(resp.Body)
	checkErr(err)

	var result Response[T]
	err = json.Unmarshal(bytes, &result)
	checkErr(err)

	return result
}

func checkErr(err error) {
	if err != nil {
		panic(err)
	}
}