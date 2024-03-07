package dip

import "testing"

const length = 1000

// (2024-03-07): 3m 18s
func BenchmarkDownloadProtokolle(b *testing.B) {
	client := NewClient()

	b.ResetTimer()

	for n := 0; n < b.N; n++ {
		documents := client.DownloadProtokolle()
		for range documents {
		} // Wait for all documents to arrive.
	}
}
