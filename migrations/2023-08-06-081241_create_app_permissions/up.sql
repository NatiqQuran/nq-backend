CREATE TABLE app_permissions (
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    creator_user_id serial NOT NULL,
    account_id serial,
    object VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT permission_fk_user_id_rel FOREIGN KEY(creator_user_id) REFERENCES app_users(id),
    CONSTRAINT permission_fk_acconut_id FOREIGN KEY(account_id) REFERENCES app_accounts(id),
    CONSTRAINT app_permission_id PRIMARY KEY (id)
);
