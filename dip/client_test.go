package dip

import "testing"

const length = 1000

// f2a41a1 - (2024-03-07) - 3m 18s - Anfang
func BenchmarkDownloadProtokolle(b *testing.B) {
	client := NewClient()

	b.ResetTimer()

	for n := 0; n < b.N; n++ {
		documents, _ := client.DownloadProtokolle()
		for range documents {
		} // Wait for all documents to arrive.
	}
}
