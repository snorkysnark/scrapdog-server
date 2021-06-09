CREATE TABLE scrapbooks(
	id INTEGER NOT NULL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE fs(
	scrapbook_id INT NOT NULL REFERENCES scrapbooks(id),
	id INT NOT NULL,
	bucket_id INT UNIQUE,

	rdf_id TEXT,

	type INT,
	title TEXT,
	source TEXT,
	icon TEXT,
	comment TEXT,
	encoding TEXT,
	marked BOOLEAN NOT NULL,
	locked BOOLEAN NOT NULL,
	created TIMESTAMP,
	modified TIMESTAMP,

	children BLOB,

	PRIMARY KEY (scrapbook_id, id)
);
