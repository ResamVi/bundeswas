package dip

import "testing"

const length = 1000

func BenchmarkGetTotalProtokoll(b *testing.B) {
	client := NewClient()

	b.ResetTimer()

	for n := 0; n < b.N; n++ {
		downloads := make(chan []PlenarprotokollText, length)
		err := client.GetTotalProtokoll(downloads, length)
		if err != nil {
			b.Error(err)
		}
	}
}
