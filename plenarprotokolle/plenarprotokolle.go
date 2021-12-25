package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"gitlab.com/resamvi/wasletztegesetz/database"

	_ "github.com/joho/godotenv/autoload"
)

var pp = "https://search.dip.bundestag.de/api/v1/plenarprotokoll?apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464&f.zuordnung=BT&f.datum.start=2021-10-26"

func main() {
	db := database.New()

	ch := fetch(pp)

	fmt.Println(ch.Documents[0])
	err := db.Store(ch.Documents[0])
	if err != nil {
		fmt.Println(err)
	}
}

func fetch(url string) Response {
	req, err := http.NewRequest(http.MethodGet, url, nil)
	if err != nil {
		panic(err)
	}

	client := http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}

	bytes, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	var result Response
	err = json.Unmarshal(bytes, &result)
	if err != nil {
		fmt.Println(string(bytes))
		panic(err)
	}
	return result
}

type Response struct {
	NumFound  int `json:"numFound"`
	Documents []Plenarprotokoll
	Cursor    string `json:"cursor"`
}

type Plenarprotokoll struct {
	Id                  string `json:"id"`
	Dokumentart         string `json:"dokumentart"`
	Typ                 string `json:"typ"`
	VorgangsbezugAnzahl int    `json:"vorgangsbezug_anzahl"`
	Dokumentnummer      string `json:"dokumentnummer"`
	Wahlperiode         int    `json:"wahlperiode"`
	Herausgeber         string `json:"herausgeber"`
	Datum               string `json:"datum"`
	Titel               string `json:"titel"`
	Fundstelle          struct {
		PdfUrl         string        `json:"pdf_url"`
		Id             string        `json:"id"`
		Dokumentnummer string        `json:"dokumentnummer"`
		Datum          string        `json:"datum"`
		Verteildatum   string        `json:"verteildatum"`
		Dokumentart    string        `json:"dokumentart"`
		Herausgeber    string        `json:"herausgeber"`
		Urheber        []interface{} `json:"urheber"`
	} `json:"fundstelle"`
	Vorgangsbezug []struct {
		Id          string `json:"id"`
		Titel       string `json:"titel"`
		Vorgangstyp string `json:"vorgangstyp"`
	} `json:"vorgangsbezug"`
}
