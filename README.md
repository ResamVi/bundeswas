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
    - [  ] Download binary mit bubble tea
        - [  ] Spinner von https://github.com/charmbracelet/bubbletea/blob/master/examples/realtime/main.go
        - [  ] Statusleiste von https://github.com/charmbracelet/bubbletea/tree/master/examples/progress-static
        - [  ] Zwischenmeldungen von https://github.com/charmbracelet/bubbletea/tree/master/examples/tui-daemon-combo
        - [  ] Endmeldung von https://github.com/charmbracelet/bubbletea/tree/master/examples/package-manager
    - [  ] Memory Usage benchmarken
- [  ] Fuzzy Finder Implementierungen
    - [  ] Interface definieren
        - [  ] Naive Text-suche durch Datei
        - [  ] Mit Postgres
        - [  ] Mit Solr
        - [  ] Mit ElasticSearch
- [  ] Wunschliste
    - [  ] Beim dritten Query in client.go schreiben wir uns helper Funktionen...
    - [  ] Responsives Frontend
