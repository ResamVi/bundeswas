-- TODO: Move Clean up commands into different file
DROP TABLE "joined";
DROP TABLE "example";


CREATE TABLE "example" (
    id int,
    name varchar,

    PRIMARY KEY(id)
);

CREATE TABLE "joined" (
    id int,
    email varchar,
    example_id int,

    PRIMARY KEY(id),
    FOREIGN KEY (example_id) REFERENCES "example"(id)
);

INSERT INTO "example" (id, name) VALUES (1, 'Julien');
INSERT INTO "example" (id, name) VALUES (2, 'Primeagen');
INSERT INTO "example" (id, name) VALUES (3, 'TJ');
INSERT INTO "example" (id, name) VALUES (4, 'Another');

INSERT INTO "joined" (id, email, example_id) VALUES (1, 'Juliens Mate', 1);
INSERT INTO "joined" (id, email, example_id) VALUES (2, 'Primes 1st mate', 2);
INSERT INTO "joined" (id, email, example_id) VALUES (3, 'Primes 2nd mate', 2);
