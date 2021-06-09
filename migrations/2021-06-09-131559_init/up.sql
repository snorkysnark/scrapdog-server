CREATE TABLE scrapbooks(
	id INTEGER NOT NULL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL
);

CREATE TABLE fs(
	id INTEGER NOT NULL PRIMARY KEY,
	is_root BOOLEAN NOT NULL,
	scrapbook_id INT NOT NULL REFERENCES scrapbooks(id),

	rdf_id TEXT,

	type INT,
	title TEXT,
	source TEXT,
	icon TEXT,
	comment TEXT,
	encoding TEXT,
	marked BOOLEAN,
	locked BOOLEAN,
	created TIMESTAMP,
	modified TIMESTAMP,

	children BLOB
);
