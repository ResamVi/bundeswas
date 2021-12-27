package main

import (
	"fmt"
	"gitlab.com/resamvi/wasletztegesetz/fetch"
	"gitlab.com/resamvi/wasletztegesetz/resource"
)


func main() {
	resp := fetch.All[resource.Plenarprotokoll](pp)

	//spew.Dump(resp)
	fmt.Println(len(resp))

}
