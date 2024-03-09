package main

import (
	"log"
	"net/http"
	"time"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /search", search)

	s := &http.Server{
		Addr:           ":8080",
		Handler:        mux,
		ReadTimeout:    10 * time.Second,
		WriteTimeout:   10 * time.Second,
		MaxHeaderBytes: 1 << 20, // 1 MB
	}

	log.Println("server started on :8080")
	log.Fatal(s.ListenAndServe())
}

func search(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello World\n"))
}
