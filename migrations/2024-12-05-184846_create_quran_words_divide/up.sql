CREATE TABLE quran_words_divide (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    word_id serial NOT NULL,
    divider_account_id serial,
    type VARCHAR(256) NOT NULL,
    CONSTRAINT word_divide_id PRIMARY KEY (id),
    CONSTRAINT fk_quran_word_divider_creator_user_id FOREIGN KEY (creator_user_id) REFERENCES app_users (id),
    CONSTRAINT fk_divide_word FOREIGN KEY (word_id) REFERENCES quran_words (id) on delete cascade,
    CONSTRAINT fk_word_divide_account_rel FOREIGN KEY (divider_account_id) REFERENCES app_accounts (id)
);
