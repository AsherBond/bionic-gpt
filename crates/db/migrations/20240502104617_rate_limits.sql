-- migrate:up

CREATE TABLE rate_limits (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    limits_role VARCHAR,
    user_email VARCHAR,
    model_id INT,
    tokens_per_hour INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_model FOREIGN KEY(model_id)
        REFERENCES models(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON rate_limits TO bionic_application;
GRANT USAGE, SELECT ON rate_limits_id_seq TO bionic_application;

-- Give access to the readonly user
GRANT SELECT ON rate_limits TO bionic_readonly;
GRANT SELECT ON rate_limits_id_seq TO bionic_readonly;


-- migrate:down

DROP TABLE rate_limits;