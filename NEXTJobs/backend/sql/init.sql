CREATE TABLE users (
    user_id INT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    is_admin BOOLEAN NOT NULL,
    is_email_verified BOOLEAN NOT NULL
);

CREATE TABLE email_verification_codes (
    user_id INT NOT NULL,
    verification_code INTEGER NOT NULL,
    expiration_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '1 hour',
    login_tmp_token VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE tokens (
    token_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    token VARCHAR(255) NOT NULL UNIQUE,
    refresh_token VARCHAR(255) NOT NULL UNIQUE,
    user_agent VARCHAR(1000) NOT NULL,
    ip_address VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    token_expiration_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '1 week',
    refresh_token_expiration_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '30 day', -- Refresh tokens typically have a longer lifespan
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    UNIQUE (user_id, user_agent, ip_address)
);
