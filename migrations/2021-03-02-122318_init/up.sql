CREATE TABLE scrapbooks(
	id INT NOT NULL PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE fs(
	scrapbook_id INT NOT NULL REFERENCES scrapbooks(id),
	id INT NOT NULL,
	is_root BOOLEAN NOT NULL,

	rdf_id TEXT,

	type INT,
	created BIGINT,
	modified BIGINT,
	source TEXT,
	icon TEXT,
	comment TEXT,
	encoding TEXT,
	marked BOOLEAN NOT NULL,
	locked BOOLEAN NOT NULL,

	children BLOB,

	PRIMARY KEY (scrapbook_id, id)
);
