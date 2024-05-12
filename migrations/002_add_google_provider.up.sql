INSERT INTO OAuth_Providers (provider_id, name)
VALUES (1, 'Google')
ON CONFLICT (provider_id) DO NOTHING;  
