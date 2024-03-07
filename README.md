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
- [  ] Handgeschriebener Client
    - [  ] "download" binary mit bubble tea
        - [x] Spinner von https://github.com/charmbracelet/bubbletea/blob/master/examples/realtime/main.go
        - [x] Statusleiste von https://github.com/charmbracelet/bubbletea/tree/master/examples/progress-static
        - [x] Zwischenmeldungen von https://github.com/charmbracelet/bubbletea/tree/master/examples/tui-daemon-combo
        - [x] Endmeldung von https://github.com/charmbracelet/bubbletea/tree/master/examples/package-manager
    - [  ] Memory Usage benchmarken
    - [x] Schnelleres fetchen von Plenarprotokoll
        - [x] Erkentniss: Man wird ziemlich schnell rate-limited. Lieber so belassen.
- [  ] Fuzzy Finder Implementierungen
    - [  ] Interface definieren
        - [  ] Naive Text-suche durch Datei
        - [  ] Mit Postgres
        - [  ] Mit Solr
        - [  ] Mit ElasticSearch
- [  ] Wunschliste
    - [  ] Beim dritten Query in client.go schreiben wir uns helper Funktionen...
    - [  ] Responsives Frontend



```
go test -run='^$' -bench=. -count=10 ./..
```


Plenarprotokolltexte nach ID
    https://search.dip.bundestag.de/api/v1/plenarprotokoll-text?apikey=rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF&f.id=1
