CREATE TABLE quran_translations_text (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    translation_id serial NOT NULL,
    ayah_id serial NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT translation_text_id PRIMARY KEY (id),
    CONSTRAINT fk_translation FOREIGN KEY(translation_id) REFERENCES quran_translations(id) on delete cascade,
    CONSTRAINT translation_text_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_ayah FOREIGN KEY(ayah_id) REFERENCES quran_ayahs(id) on delete cascade
);
