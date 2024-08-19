CREATE TABLE quran_surahs (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    name VARCHAR(50) NOT NULL,
    period VARCHAR(50),
    number serial NOT NULL,
    bismillah_status BOOLEAN NOT NULL,
    bismillah_as_first_ayah BOOLEAN NOT NULL,
    mushaf_id serial NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT quran_surahs_id PRIMARY KEY (id),
    CONSTRAINT surah_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT fk_mushaf_id FOREIGN KEY(mushaf_id) REFERENCES quran_mushafs(id)
        on delete cascade
);
