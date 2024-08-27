CREATE TABLE app_phrase_translations (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    phrase_id serial NOT NULL,
    text TEXT NOT NULL,
    language VARCHAR(3) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_phrase_translations_id PRIMARY KEY (id),
    CONSTRAINT app_phrase_translations_fk_phrase FOREIGN KEY(phrase_id) REFERENCES app_phrases(id) on delete cascade
);
