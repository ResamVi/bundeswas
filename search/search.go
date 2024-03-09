package search

import "github.com/resamvi/bundeswas/dip"

// Searcher implements fuzzy finding on Plenarprotokolle.
type Searcher interface {
	// Insert a new Plenarprotokoll that will be part of the search.
	Insert(protokoll dip.PlenarprotokollText)

	// Search for a term.
	Search(term string) []Result
}

// Result are sentences that should closely match the term.
type Result struct {
	Position string
	Sentence string
}
