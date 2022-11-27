1. Vorgänge in Tabelle erfassen um
- zu gucken welche Felder immer vorhanden sind.
- welche Typen von Vorgängen es gibt (nicht jeder Vorgang hat ein abstract)
- Was der Unterschied zwischen einem Vorgang und einer Aktivität ist

Features

- Alle Vorgänge vom Vorgangstyp "Antrag" bei /vorgang suchen
- Um Parsen zu können wie abgestimmt wurde /vorgangsposition
"beschlussfassung": [
    {
        "beschlusstenor": "Ablehnung der Vorlage",
        "dokumentnummer": "20/4339",
        "seite": "7426A"
    }
]]

Plenarprotokoll -> vorgang?f.plenarprotokoll={id} -> aktivitaet.vorgangsbezug[].id

TODOs
- Gesamtzahlen was evaluiert wurde (Aktivitäten, Plenarprotokolle, Vorgänge, Drucksachen)
- Count down bis zur nächsten Sitzung
- Man beobachtet dass ein Plenarprotokoll vorhanden ist aber noch kein Link, keine Vorgänge:
```
{
    "titel": "Protokoll der 71. Sitzung des 20. Deutschen Bundestages",
    "datum": "2022-11-25",
    "dokumentnummer": "20/71",
    "pdf_url": "https://dserver.bundestag.de/btp/20/20071.pdf",
    "vorgaenge": []
}
```

