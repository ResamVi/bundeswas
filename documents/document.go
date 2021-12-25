package documents

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

var url = "https://search.dip.bundestag.de/api/v1/aktivitaet?apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464"

var trunk = "https://search.dip.bundestag.de/api/v1/"
var apiKey = "apikey=N64VhW8.yChkBUIJeosGojQ7CSR2xwLf3Qy7Apw464"
var zuordnung = "f.zuordnung=BT"

// aktivitaet drucksache-text plenarprotokolltext vorgang

var start = "f.datum.start=2021-12-23"
var plenarprotokoll = "f.drucksache=68852"

func Fetch() chan []Document {
	cursor := url

	ch := make(chan []Document, 100)

	go func() {
		for {
			resp := fetch(cursor)
			ch <- resp.Documents

			// TODO: break when length == 0
			if resp.Cursor == "" {
				break
			}
			resp.Cursor = strings.ReplaceAll(resp.Cursor, "/", "%2F")
			resp.Cursor = strings.ReplaceAll(resp.Cursor, "+", "%2B")
			fmt.Println(resp.Cursor)

			cursor = fmt.Sprintf("%s&cursor=%s", url, resp.Cursor)
			time.Sleep(2 * time.Second)

		}
	}()

	return ch
}

type Response struct {
	NumFound  int        `json:"numFound"`
	Documents []Document `json:"documents"`
	Cursor    string     `json:"cursor"`
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

type Document struct {
	Id string `json:"id"`

	// :- { Kleine Anfrage, Antrag, Frage, Antwort, Rede, Berichterstattung, Entschließungsantrag, Änderungsantrag, Gesetzentwurf, Schriftliche Erklärung gem. § 31 Geschäftsordnung BT, Rede (zu Protokoll gegeben), Zwischenfrage, Zusatzfrage, Kurzintervention, Erwiderung, Zur Geschäftsordnung BT, Große Anfrage}
	Aktivitaetsart      string `json:"aktivitaetsart"`
	Typ                 string `json:"typ"`
	VorgangsbezugAnzahl int    `json:"vorgangsbezug_anzahl"`
	// Bei Rede gibts deskriptor attribute

	// :- { Plenarprotokoll, Drucksache }
	Dokumentart   string          `json:"dokumentart"`
	Wahlperiode   int             `json:"wahlperiode"`
	Datum         string          `json:"datum"`
	Titel         string          `json:"titel"`
	Fundstelle    Fundstelle      `json:"fundstelle" gorm:"foreignKey:Id;references:Id"`
	Vorgangsbezug []Vorgangsbezug `json:"vorgangsbezug" gorm:"foreignKey:DocumentID;references:Id"`
}

type Fundstelle struct {
	Id             string  `json:"id"`
	PdfUrl         string  `json:"pdf_url"`
	Dokumentnummer string  `json:"dokumentnummer"`
	Datum          string  `json:"datum"`
	Dokumentart    string  `json:"dokumentart"`
	Drucksachetyp  string  `json:"drucksachetyp"`
	Herausgeber    string  `json:"herausgeber"`
	Urheber        Urheber `json:"urheber"`
	Verteildatum   string  `json:"verteildatum"`
}

type Vorgangsbezug struct {
	Vorgangsposition string `json:"vorgangsposition"`
	Vorgangstyp      string `json:"vorgangstyp"`
	Titel            string `json:"titel"`
	Id               string `json:"id"`
	DocumentID       string `json:"-"`
}

type Urheber []string

func (u *Urheber) GormDataType() string {
	return "text"
}

func (u *Urheber) Scan(value interface{}) error {
	bytes, ok := value.([]byte)
	if !ok {
		return errors.New(fmt.Sprint("Failed to unmarshal JSONB value:", value))
	}

	var result []string
	json.Unmarshal(bytes, &result)

	*u = result

	return nil
}

func (u Urheber) Value() (driver.Value, error) {
	if len(u) == 0 {
		return nil, nil
	}

	return fmt.Sprintf("%v", u), nil
}
