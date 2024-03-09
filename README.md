# Bundeswas

Google-ähnliche Autovervollständigung für Zitate von Bundestags-Politikern (technisch: Plenarprotokoll fuzzy finder).

# Technisches

Benutzt die [API des Dokumentations- und Informationssystems für Parlamentsmaterialien](https://search.dip.bundestag.de/api/v1/swagger-ui/) siehe Kopie der Spezifikation in `[openapi.yaml](openapi.yaml)`.

# Features

- Textbox mit Autocomplete
- Beim Finden eines Zitats navigiere zum entsprechenden Teil des Plenarprotokolls

# TODOs
- [  ] Frontend
    - [x] Use template
    - [  ] Retrieve results from backend
- [x] Handgeschriebener Client
    - [x] "download" binary mit bubble tea
        - [x] Spinner von https://github.com/charmbracelet/bubbletea/blob/master/examples/realtime/main.go
        - [x] Statusleiste von https://github.com/charmbracelet/bubbletea/tree/master/examples/progress-static
        - [x] Zwischenmeldungen von https://github.com/charmbracelet/bubbletea/tree/master/examples/tui-daemon-combo
        - [x] Endmeldung von https://github.com/charmbracelet/bubbletea/tree/master/examples/package-manager
        - [ ] Choice von https://github.com/charmbracelet/bubbletea/tree/master/examples/result
        - [ ] Zeitmessung von https://github.com/charmbracelet/bubbletea/tree/master/examples/stopwatch
        - [ ] Zwei Ansichten von https://github.com/charmbracelet/bubbletea/tree/master/examples/views
    - [x] Schnelleres fetchen von Plenarprotokoll
        - [x] Erkentniss: Man wird ziemlich schnell rate-limited. Lieber so belassen.
- [  ] Fuzzy Finder Implementierungen
    - [x] Interface definieren
        - [  ] Naive Text-suche durch Datei
        - [  ] Mit Postgres
        - [  ] Mit Solr
        - [  ] Mit ElasticSearch
- [  ] Server mit API
    - [  ] Websocket Endpunkt
    - [  ] GET HTTP Endpunkt
    - [  ] CORS
    - [  ] Setup mit Environment Variables
    - [  ] Containerisiert

- [  ] Wunschliste
    - [  ] Beim dritten Query in client.go schreiben wir uns helper Funktionen...
    - [  ] Responsives Frontend
    - [  ] Memory Usage benchmarken



```
go test -run='^$' -bench=. -count=10 ./..
```


Plenarprotokolltexte nach ID
    https://search.dip.bundestag.de/api/v1/plenarprotokoll-text?apikey=rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF&f.id=1
