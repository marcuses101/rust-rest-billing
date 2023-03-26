CREATE TABLE coaches (
    id SERIAL PRIMARY KEY,
    solo_rate_cents_per_minute INTEGER NOT NULL,
    group_rate_cents_per_minute INTEGER NOT NULL,
    tax_rate_percent DECIMAL(5,2) NOT NULL,
    company_name TEXT NOT NULL,
    company_street TEXT NOT NULL,
    company_town TEXT NOT NULL,
    company_province TEXT NOT NULL,
    company_country TEXT NOT NULL,
    company_postal_code TEXT NOT NULL,
    sales_tax_number TEXT
);