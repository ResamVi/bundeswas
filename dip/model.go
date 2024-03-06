// Package dip provides a client to interact with HTTP API of DIP (Dokumentations- und Informationssystems für Parlamentsmaterialien).
package dip

import (
	"fmt"
	"time"

	"github.com/oapi-codegen/runtime/types"
)

// Quadrant Teil der Fundstelle eines Plenarprotokolls. Jede Seite im Plenarprotokoll ist in vier gleich große Viertel unterteilt (Quadranten) mit den Bezeichnungen A, B, C, D.
type Quadrant string

// Defines values for Quadrant.
const (
	A Quadrant = "A"
	B Quadrant = "B"
	C Quadrant = "C"
	D Quadrant = "D"
)

// Zuordnung Jeder Vorgangsschritt ist entweder dem Bundestag (BT), dem Bundesrat (BR), der Bundesversammlung (BV) oder der Europakammer (EK) zugeordnet. Über die Zuordnung lassen sich bspw. Rechtsverordnungen herausfiltern, an denen der Bundestag beteiligt / nicht beteiligt war.
type Zuordnung string

// Defines values for Zuordnung.
const (
	BR Zuordnung = "BR"
	BT Zuordnung = "BT"
	BV Zuordnung = "BV"
	EK Zuordnung = "EK"
)

// Fundstelle Liefert im Vorgangsablauf das zu einem Vorgangsschritt gehörende Dokument (Drucksache oder Protokoll).
//
// Beispiel: „BT-Drucksache 19/1 (Antrag Fraktion der CDU/CSU)“ oder beim Vorgangsschritt Beratung „BT-Plenarprotokoll 19/1, S. 4C-12A“.
type Fundstelle struct {
	// Anfangsquadrant Teil der Fundstelle eines Plenarprotokolls. Jede Seite im Plenarprotokoll ist in vier gleich große Viertel unterteilt (Quadranten) mit den Bezeichnungen A, B, C, D.
	Anfangsquadrant *Quadrant             `json:"anfangsquadrant,omitempty"`
	Anfangsseite    *int                  `json:"anfangsseite,omitempty"`
	Anlagen         *string               `json:"anlagen,omitempty"`
	Datum           types.Date            `json:"datum"`
	Dokumentart     FundstelleDokumentart `json:"dokumentart"`
	Dokumentnummer  string                `json:"dokumentnummer"`
	Drucksachetyp   *string               `json:"drucksachetyp,omitempty"`

	// Endquadrant Teil der Fundstelle eines Plenarprotokolls. Jede Seite im Plenarprotokoll ist in vier gleich große Viertel unterteilt (Quadranten) mit den Bezeichnungen A, B, C, D.
	Endquadrant *Quadrant `json:"endquadrant,omitempty"`
	Endseite    *int      `json:"endseite,omitempty"`
	FrageNummer *string   `json:"frage_nummer,omitempty"`

	// Herausgeber Jeder Vorgangsschritt ist entweder dem Bundestag (BT), dem Bundesrat (BR), der Bundesversammlung (BV) oder der Europakammer (EK) zugeordnet. Über die Zuordnung lassen sich bspw. Rechtsverordnungen herausfiltern, an denen der Bundestag beteiligt / nicht beteiligt war.
	Herausgeber Zuordnung `json:"herausgeber"`

	// Id ID einer Drucksache oder eines Plenarprotokolls
	Id           string      `json:"id"`
	PdfUrl       *string     `json:"pdf_url,omitempty"`
	Seite        *string     `json:"seite,omitempty"`
	Top          *int32      `json:"top,omitempty"`
	TopZusatz    *string     `json:"top_zusatz,omitempty"`
	Urheber      []string    `json:"urheber"`
	Verteildatum *types.Date `json:"verteildatum,omitempty"`
}

// FundstelleDokumentart defines model for Fundstelle.Dokumentart.
type FundstelleDokumentart string

// PlenarprotokollText defines model for PlenarprotokollText.
type PlenarprotokollText struct {
	// Aktualisiert Letzte Aktualisierung der Entität
	Aktualisiert time.Time  `json:"aktualisiert"`
	Datum        types.Date `json:"datum"`

	// Immer "Plenarprotokoll"
	Dokumentart    string `json:"dokumentart"`
	Dokumentnummer string `json:"dokumentnummer"`

	// Fundstelle Liefert im Vorgangsablauf das zu einem Vorgangsschritt gehörende Dokument (Drucksache oder Protokoll).
	//
	// Beispiel: „BT-Drucksache 19/1 (Antrag Fraktion der CDU/CSU)“ oder beim Vorgangsschritt Beratung „BT-Plenarprotokoll 19/1, S. 4C-12A“.
	Fundstelle Fundstelle `json:"fundstelle"`

	// Herausgeber Jeder Vorgangsschritt ist entweder dem Bundestag (BT), dem Bundesrat (BR), der Bundesversammlung (BV) oder der Europakammer (EK) zugeordnet. Über die Zuordnung lassen sich bspw. Rechtsverordnungen herausfiltern, an denen der Bundestag beteiligt / nicht beteiligt war.
	Herausgeber Zuordnung `json:"herausgeber"`
	Id          string    `json:"id"`

	// PdfHash MD5-Prüfsumme der PDF-Datei
	PdfHash           *string `json:"pdf_hash,omitempty"`
	Sitzungsbemerkung *string `json:"sitzungsbemerkung,omitempty"`

	// Text Volltext des Dokuments
	//
	// Das Beispiel enthält einen gekürzten Auszug einer Drucksache.
	Text  *string `json:"text,omitempty"`
	Titel string  `json:"titel"`

	// Immer "Dokument"
	Typ string `json:"typ"`

	// Vorgangsbezug Zusammenfassung der ersten 4 zugehörigen Vorgänge
	Vorgangsbezug *[]Vorgangsbezug `json:"vorgangsbezug,omitempty"`

	// VorgangsbezugAnzahl Gesamtzahl der zugehörigen Vorgänge
	VorgangsbezugAnzahl int32  `json:"vorgangsbezug_anzahl"`
	Wahlperiode         *int32 `json:"wahlperiode,omitempty"`
}

// PlenarprotokollTextListResponse defines model for PlenarprotokollTextListResponse.
type Response struct {
	Cursor    string                `json:"cursor"`
	Documents []PlenarprotokollText `json:"documents"`
	NumFound  int                   `json:"numFound"`
}

// Vorgangsbezug Liefert ID, Titel und Vorgangstyp eines Vorgangs, der mit der Drucksache oder dem Plenarprotokoll verbunden ist.
type Vorgangsbezug struct {
	// Id ID eines verknüpften Vorgangs
	Id          string `json:"id"`
	Titel       string `json:"titel"`
	Vorgangstyp string `json:"vorgangstyp"`
}

// ErrorResponse (entweder 400, 401, 404)
type ErrorResponse struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

func (e ErrorResponse) Error() string {
	return fmt.Sprintf("[Code: '%v', Message: '%v']", e.Code, e.Message)
}
