/* [074A-13] Tabla de miembros del equipo para CMS */
CREATE TABLE IF NOT EXISTS team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL UNIQUE,
    role VARCHAR(200) NOT NULL DEFAULT '',
    bio TEXT NOT NULL DEFAULT '',
    avatar VARCHAR(500) DEFAULT NULL,
    linkedin VARCHAR(500) DEFAULT NULL,
    twitter VARCHAR(500) DEFAULT NULL,
    github VARCHAR(500) DEFAULT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'published',
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_team_members_status ON team_members(status);
CREATE INDEX idx_team_members_sort ON team_members(sort_order);
