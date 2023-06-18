CREATE TABLE IF NOT EXISTS money_transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    destination VARCHAR(255) NOT NULL,
    source VARCHAR(255) NOT NULL,
    amount INTEGER NOT NULL,
    nominal VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS transaction_entities (
    id SERIAL PRIMARY KEY,
    transaction_entity VARCHAR(255) NOT NULL,
    transaction_entity_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);