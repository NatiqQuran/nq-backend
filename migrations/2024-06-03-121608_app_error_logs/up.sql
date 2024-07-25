CREATE TABLE app_error_logs(
    id serial NOT NULL,
    uuid uuid DEFAULT uuid_generate_v4 () NOT NULL,
    error_name VARCHAR(256) NOT NULL,
    status_code INT NOT NULL,
    message TEXT NOT NULL,
    detail TEXT,
    account_id INT,
    request_token VARCHAR(64),
    request_user_agent TEXT,
    request_ipv4 CIDR NOT NULL,
    request_url TEXT,
    request_controller TEXT,
    request_action TEXT,
    request_id TEXT,
    request_body BYTEA,
    request_body_content_type TEXT, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT app_error_logs_id PRIMARY KEY (id)
);

