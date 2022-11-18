1. Vorgänge in Tabelle erfassen um
- zu gucken welche Felder immer vorhanden sind.
- welche Typen von Vorgängen es gibt (nicht jeder Vorgang hat ein abstract)
- Was der Unterschied zwischen einem Vorgang und einer Aktivität ist

Features
- Count down bis zur nächsten Sitzung
- Gesamtzahlen was evaluiert wurde (Aktivitäten, Plenarprotokolle, Vorgänge, Drucksachen)

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

Vorgang
