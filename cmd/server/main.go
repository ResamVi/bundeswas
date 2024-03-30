package main

import (
	"context"
	"log"
	"net/http"
	"time"

	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wsjson"
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
	c, err := websocket.Accept(w, r, &websocket.AcceptOptions{
		OriginPatterns: []string{"*"},
	})

	if err != nil {
		panic(err)
	}
	defer c.CloseNow()

	for {
		var v string
		err = wsjson.Read(context.Background(), c, &v)
		if err != nil {
			panic(err)
		}
		log.Printf("received: %v\n", v)

		err = wsjson.Write(context.Background(), c, "hi")
		if err != nil {
			panic(err)
		}
	}

	// c.Close(websocket.StatusNormalClosure, "")
}
