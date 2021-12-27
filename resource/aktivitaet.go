package resource

type Aktivitaet struct {
	Id                  string `json:"id"`
	Aktivitaetsart      string `json:"aktivitaetsart"`
	Typ                 string `json:"typ"`
	VorgangsbezugAnzahl int    `json:"vorgangsbezug_anzahl"`
	Dokumentart         string `json:"dokumentart"`
	Wahlperiode         int    `json:"wahlperiode"`
	Datum               string `json:"datum"`
	Titel               string `json:"titel"`
	Fundstelle          struct {
		Seite           string        `json:"seite"`
		PdfUrl          string        `json:"pdf_url"`
		Id              string        `json:"id"`
		Dokumentnummer  string        `json:"dokumentnummer"`
		Datum           string        `json:"datum"`
		Verteildatum    string        `json:"verteildatum"`
		Dokumentart     string        `json:"dokumentart"`
		Herausgeber     string        `json:"herausgeber"`
		Urheber         []interface{} `json:"urheber"`
		Anfangsseite    int           `json:"anfangsseite"`
		Endseite        int           `json:"endseite"`
		Anfangsquadrant string        `json:"anfangsquadrant"`
		Endquadrant     string        `json:"endquadrant"`
	} `json:"fundstelle"`
	Vorgangsbezug []struct {
		Vorgangsposition string `json:"vorgangsposition"`
		Vorgangstyp      string `json:"vorgangstyp"`
		Titel            string `json:"titel"`
		Id               string `json:"id"`
	} `json:"vorgangsbezug"`
	Deskriptor []struct {
		Name string `json:"name"`
		Typ  string `json:"typ"`
	} `json:"deskriptor,omitempty"`
}
