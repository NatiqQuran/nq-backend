CREATE TABLE quran_ayahs_divide (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    ayah_id serial NOT NULL,
    divider_account_id serial,
    type VARCHAR(256) NOT NULL,
    CONSTRAINT ayah_divide_id PRIMARY KEY (id),
    CONSTRAINT fk_quran_ayah_divider_creator_user_id FOREIGN KEY (creator_user_id) REFERENCES app_users (id),
    CONSTRAINT fk_divide_ayah FOREIGN KEY (ayah_id) REFERENCES quran_ayahs (id) on delete cascade,
    CONSTRAINT fk_ayah_divide_account_rel FOREIGN KEY (divider_account_id) REFERENCES app_accounts (id)
);
