package dip

import "testing"

const length = 1000

func BenchmarkDownloadProtokolle(b *testing.B) {
	client := NewClient()

	b.ResetTimer()

	for n := 0; n < b.N; n++ {
		documents := client.DownloadProtokolle(length)
		for range documents {
		} // Wait for all documents to arrive.
	}
}
