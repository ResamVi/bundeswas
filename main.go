package main

import (
	"fmt"

	"gitlab.com/resamvi/wasletztegesetz/database"

	_ "github.com/joho/godotenv/autoload"

	"gitlab.com/resamvi/wasletztegesetz/documents"
)

func main() {
	db := database.New()
	ch := documents.Fetch()

	count := 0
	for docs := range ch {
		count += len(docs)
		fmt.Println(count)

		for _, d := range docs {
			err := db.Create(&d).Error
			if err != nil {
				fmt.Println(err)
			}
		}
	}
	//var user documents.Document
	//db.First(&user)

}
