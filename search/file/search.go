package file

import (
	"github.com/resamvi/bundeswas/dip"
	"github.com/resamvi/bundeswas/search"
)

type fileSearch struct {
}

func NewSearch() *fileSearch {
	return &fileSearch{}
}

func (f fileSearch) Insert(protokoll dip.PlenarprotokollText) {

}

// Search for a term.
func (f fileSearch) Search(term string) []search.Result {
	return nil
}
