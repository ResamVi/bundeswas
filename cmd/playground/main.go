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
	for range documents {
	} // Wait for all documents to arrive.

	fmt.Println(time.Since(start))
}
