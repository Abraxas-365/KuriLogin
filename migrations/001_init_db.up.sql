-- Creating the Users table
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NULL,
    name VARCHAR(255) NULL,
    avatar_url TEXT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Creating the OAuth_Providers table
CREATE TABLE OAuth_Providers (
    provider_id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Creating the OAuth_Authorizations table
CREATE TABLE OAuth_Authorizations (
    auth_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    provider_id INTEGER NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT NULL,
    expires_in TIMESTAMP WITH TIME ZONE NULL,
    scope TEXT[] NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES Users(user_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_provider
        FOREIGN KEY(provider_id)
        REFERENCES OAuth_Providers(provider_id)
        ON DELETE CASCADE
  CONSTRAINT unq_provider_user_id UNIQUE (provider_user_id)
);

-- Indexes for improved performance on frequent queries
CREATE INDEX idx_oauth_user_id ON OAuth_Authorizations (user_id);
CREATE INDEX idx_oauth_provider_id ON OAuth_Authorizations (provider_id);
CREATE INDEX idx_user_email ON Users (email);
