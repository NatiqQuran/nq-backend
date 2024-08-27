CREATE TABLE app_phrases (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    phrase TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_phrase_id PRIMARY KEY (id)
);
