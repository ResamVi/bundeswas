package resource

type Vorgang struct {
	Id                        string   `json:"id"`
	Beratungsstand            string   `json:"beratungsstand,omitempty"`
	Vorgangstyp               string   `json:"vorgangstyp"`
	Gesta                     string   `json:"gesta,omitempty"`
	Sachgebiet                []string `json:"sachgebiet,omitempty"`
	Typ                       string   `json:"typ"`
	Wahlperiode               int      `json:"wahlperiode"`
	Zustimmungsbeduerftigkeit []string `json:"zustimmungsbeduerftigkeit,omitempty"`
	Initiative                []string `json:"initiative,omitempty"`
	Datum                     string   `json:"datum"`
	Titel                     string   `json:"titel"`
	Deskriptor                []struct {
		Fundstelle bool   `json:"fundstelle"`
		Name       string `json:"name"`
		Typ        string `json:"typ"`
	} `json:"deskriptor,omitempty"`
	Abstract string `json:"abstract,omitempty"`
}
