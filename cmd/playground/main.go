package main

import (
	"fmt"

	"github.com/resamvi/bundeswas/dip"
)

func main() {
	client := dip.NewClient()

	documents := client.DownloadProtokolle(100)
	for i := range documents {
		fmt.Println(i.Id)
	} // Wait for all documents to arrive.

	fmt.Println("done")
}
