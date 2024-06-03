CREATE TABLE app_error_logs(
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    error_name VARCHAR(256) NOT NULL,
    status_code INT NOT NULL,
    --- TODO: ADD user IP and ...
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_error_logs_id PRIMARY KEY (id)
);

