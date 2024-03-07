package main

import (
	"fmt"
	"time"

	"github.com/resamvi/bundeswas/dip"
)

func main() {
	client := dip.NewClient()

	start := time.Now()
	documents := client.DownloadProtokolle()
	for document := range documents {
		fmt.Println(document.Id)
	}

	fmt.Println(time.Since(start))
}
