package resource

import (
	"fmt"
	_ "github.com/joho/godotenv/autoload"
	"gitlab.com/resamvi/wasletztegesetz/fetch"
	"sort"
)

type Plenarprotokoll struct {
	Id                  string `json:"id"`
	//VorgangsbezugAnzahl int    `json:"vorgangsbezug_anzahl"` 	// = len(Vorgangspositionen) approx.
	Dokumentnummer      string `json:"dokumentnummer"`
	Datum               string `json:"datum"`
	Titel               string `json:"titel"`
	//Dokumentart         string `json:"dokumentart"` 			// Always "Plenarprotokoll"
	//Typ                 string `json:"typ"` 					// Always "Dokument"
	//Herausgeber string `json:"herausgeber"` 					// We only focus on the Bundestag (Always "BT")
	//Wahlperiode         int    `json:"wahlperiode"` 			// We only focus on the 20th period (Always "20")
	Fundstelle struct {
		PdfUrl         string        `json:"pdf_url"`
		Id             string        `json:"id"`
		Dokumentnummer string        `json:"dokumentnummer"`
		Verteildatum   string        `json:"verteildatum"`
		Herausgeber    string        `json:"herausgeber"`
		Urheber        []interface{} `json:"urheber"` // TODO: Is this ever not empty?
		//Datum          string        `json:"datum"` 			// Same as parent
		//Dokumentart    string        `json:"dokumentart"` 	// Always "Plenarprotokoll"
	} `json:"fundstelle"`
	//Vorgangsbezug []struct {									// I don't get why this only has a select few.
		//Id          string `json:"id"`
		//Titel       string `json:"titel"`
		//Vorgangstyp string `json:"vorgangstyp"`
	//} `json:"vorgangsbezug"`

	//Vorgaenge          []Vorgang          `json:"vorgaenge"`
	//Aktivitaeten       []Aktivitaet       `json:"aktivitaeten"`
	Vorgangspositionen []Vorgangsposition `json:"vorgangspositionen"`
}

var pp = "https://search.dip.bundestag.de/api/v1/plenarprotokoll?apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464&f.zuordnung=BT&f.datum.start=2021-10-26"

var vorgang = func(id string) string {
	return fmt.Sprintf("https://search.dip.bundestag.de/api/v1/vorgang?f.plenarprotokoll=%s&apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464", id)
}

var aktiv = func(id string) string {
	return fmt.Sprintf("https://search.dip.bundestag.de/api/v1/aktivitaet?f.plenarprotokoll=%s&apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464", id)
}

var pos = func(id string) string {
	return fmt.Sprintf("https://search.dip.bundestag.de/api/v1/vorgangsposition?f.plenarprotokoll=%s&apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464", id)
}

var single_vorgang = func(id string) string {
	return fmt.Sprintf("https://search.dip.bundestag.de/api/v1/vorgang/%s?apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464", id)
}

func Fetch() []Plenarprotokoll {
	protokolle := fetch.All[Plenarprotokoll](pp)

	for i := range protokolle[:3] {
		id := protokolle[i].Id

		//vorgaenge := fetch.All[Vorgang](vorgang(id))
		//aktivitaeten := fetch.All[Aktivitaet](aktiv(id))
		vorgangspositionen := fetch.All[Vorgangsposition](pos(id))

		for i := range vorgangspositionen {
			vorgang := fetch.One[Vorgang](single_vorgang(vorgangspositionen[i].VorgangId))
			vorgangspositionen[i].Vorgang = vorgang
		}

		sort.Slice(vorgangspositionen, func(i, j int) bool {
			return vorgangspositionen[i].Id < vorgangspositionen[j].Id
		})
		//sort.Slice(aktivitaeten, func(i, j int) bool {
		//	return aktivitaeten[i].Id > aktivitaeten[j].Id
		//})

		//protokolle[i].Vorgaenge = vorgaenge
		protokolle[i].Vorgangspositionen = vorgangspositionen
		//protokolle[i].Aktivitaeten = aktivitaeten
	}

	return protokolle
}

func reverse[T any](s []T) []T {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
	return s
}