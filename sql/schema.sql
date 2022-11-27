DROP TABLE IF EXISTS "vorgangsbezuege";
DROP TABLE IF EXISTS "aktivitaeten";
DROP TABLE IF EXISTS "fundstellen_urheber";
DROP TABLE IF EXISTS "fundstellen";
DROP TABLE IF EXISTS "plenarprotokolle";

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

    PRIMARY KEY(id)
);

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

