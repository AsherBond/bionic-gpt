-- migrate:up
CREATE TABLE oauth_clients (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    provider TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON oauth_clients TO bionic_application;
GRANT USAGE, SELECT ON oauth_clients_id_seq TO bionic_application;

-- Give access to the readonly user
GRANT SELECT ON oauth_clients TO bionic_readonly;
GRANT SELECT ON oauth_clients_id_seq TO bionic_readonly;

-- migrate:down
DROP TABLE oauth_clients;