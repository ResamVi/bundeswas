-- TODO: Move Clean up commands into different file
-- {
--  "id": "1618994",
--  "aktivitaetsart": "Kleine Anfrage",
--  "typ": "Aktivität",
--  "vorgangsbezug_anzahl": 1,
--  "dokumentart": "Drucksache",
--  "wahlperiode": 20,
--  "datum": "2022-11-15",
--  "titel": "Pascal Meiser, MdB, DIE LINKE",
--  "fundstelle": {
--    "pdf_url": "https://dserver.bundestag.de/btd/20/044/2004473.pdf",
--    "id": "264348",
--    "dokumentnummer": "20/4473",
--    "datum": "2022-11-15",
--    "dokumentart": "Drucksache",
--    "drucksachetyp": "Kleine Anfrage",
--    "herausgeber": "BT",
--    "urheber": [
--      "Fraktion DIE LINKE"
--    ]
--  },
--  "vorgangsbezug": [
--    {
--      "vorgangsposition": "Kleine Anfrage",
--      "vorgangstyp": "Kleine Anfrage",
--      "titel": "Aktuelle Entwicklungen in der Leiharbeit",
--      "id": "293472"
--    }
--  ]
-- }
DROP TABLE IF EXISTS "vorgangsbezuege";
DROP TABLE IF EXISTS "aktivitaeten";
DROP TABLE IF EXISTS "fundstellen_urheber";
DROP TABLE IF EXISTS "fundstellen";

CREATE TABLE "fundstellen" (
    pdf_url         varchar,
    id              varchar,
    dokumentnummer  varchar,
    datum           varchar, -- TODO:
    dokumentart     varchar,
    drucksachetyp   varchar, -- sometimes null
    herausgeber     varchar, -- always BT
    verteildatum    varchar, -- sometimes null
    -- urheber always empty?

    PRIMARY KEY(id)
);

CREATE TABLE "fundstellen_urheber" (
    fundstelle_id varchar,
    urheber varchar,

    FOREIGN KEY(fundstelle_id) REFERENCES "fundstellen"(id)
);

CREATE TABLE "aktivitaeten" (
    id varchar,
    aktivitaetsart varchar,
    typ varchar,
    vorgangsbezug_anzahl int,
    dokumentart varchar,
    wahlperiode int,
    datum varchar, -- TODO: 
    titel varchar,
    fundstelle_id varchar,

    PRIMARY KEY(id),
    FOREIGN KEY (fundstelle_id) REFERENCES "fundstellen"(id)
);

CREATE TABLE "vorgangsbezuege" (
    id varchar,
    vorgangsposition varchar,
    vorgangstyp varchar, 
    titel varchar,

    PRIMARY KEY(id),
);

-- Join Table

CREATE TABLE "plenarprotokolle" (
    id                      varchar,
    dokumentart             varchar,
    typ                     varchar,
    vorgangsbezug_anzahl    int,
    dokumentnummer          varchar,
    wahlperiode             int, -- always the same
    herausgeber             varchar, -- always the same
    datum                   varchar,
    titel                   varchar,
    fundstelle_id           varchar
);


/* INSERT INTO "example" (id, name) VALUES (1, 'Julien'); */
/* INSERT INTO "example" (id, name) VALUES (2, 'Primeagen'); */
/* INSERT INTO "example" (id, name) VALUES (3, 'TJ'); */
/* INSERT INTO "example" (id, name) VALUES (4, 'Another'); */

/* INSERT INTO "joined" (id, email, example_id) VALUES (1, 'Juliens Mate', 1); */
/* INSERT INTO "joined" (id, email, example_id) VALUES (2, 'Primes 1st mate', 2); */
/* INSERT INTO "joined" (id, email, example_id) VALUES (3, 'Primes 2nd mate', 2); */

