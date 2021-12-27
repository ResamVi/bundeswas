package resource

type Vorgangsposition struct {
	Id                string `json:"id"`
	Vorgangsposition  string `json:"vorgangsposition"`
	//Zuordnung         string `json:"zuordnung"`		// Always "BT"
	Gang              bool   `json:"gang"`
	Fortsetzung       bool   `json:"fortsetzung"`
	Nachtrag          bool   `json:"nachtrag"`
	Vorgangstyp       string `json:"vorgangstyp"`
	//Typ               string `json:"typ"`				// Always "Vorgangsposition"
	Titel             string `json:"titel"`
	AktivitaetAnzahl  int    `json:"aktivitaet_anzahl"`
	//Dokumentart       string `json:"dokumentart"`		// Always "Plenarprotokoll"
	VorgangId         string `json:"vorgang_id"`
	//Datum             string `json:"datum"` 			// Redundant. Already found in Plenarprotokoll
	AktivitaetAnzeige []struct {
		Seite          string `json:"seite"`
		Aktivitaetsart string `json:"aktivitaetsart"`
		Titel          string `json:"titel"`
		PdfUrl         string `json:"pdf_url"`
	} `json:"aktivitaet_anzeige,omitempty"`
	Fundstelle struct {
		PdfUrl          string        `json:"pdf_url,omitempty"`
		Id              string        `json:"id,omitempty"`
		Dokumentnummer  string        `json:"dokumentnummer,omitempty"`
		Datum           string        `json:"datum,omitempty"`
		Verteildatum    string        `json:"verteildatum,omitempty"`
		Dokumentart     string        `json:"dokumentart,omitempty"`
		Herausgeber     string        `json:"herausgeber,omitempty"`
		Urheber         []interface{} `json:"urheber,omitempty"`
		Anfangsseite    int           `json:"anfangsseite,omitempty"`
		Endseite        int           `json:"endseite,omitempty"`
		Anfangsquadrant string        `json:"anfangsquadrant,omitempty"`
		Endquadrant     string        `json:"endquadrant,omitempty"`
	} `json:"fundstelle"`
	Beschlussfassung []struct {
		Beschlusstenor           string `json:"beschlusstenor"`
		Seite                    string `json:"seite"`
		AbstimmErgebnisBemerkung string `json:"abstimm_ergebnis_bemerkung,omitempty"`
		Dokumentnummer           string `json:"dokumentnummer,omitempty"`
	} `json:"beschlussfassung,omitempty"`
	Abstract     string `json:"abstract,omitempty"`
	Ueberweisung []struct {
		Ausschuss        string `json:"ausschuss"`
		AusschussKuerzel string `json:"ausschuss_kuerzel"`
		Federfuehrung    bool   `json:"federfuehrung"`
	} `json:"ueberweisung,omitempty"`
	Mitberaten []struct {
		Vorgangsposition string `json:"vorgangsposition"`
		Vorgangstyp      string `json:"vorgangstyp"`
		Titel            string `json:"titel"`
		Id               string `json:"id"`
	} `json:"mitberaten,omitempty"`

	Vorgang Vorgang `json:"vorgang"`
}
