-- Up Migration: Initial Schema for Fuksiseikkailu with Unified Users & OTP Auth

-- 1. Enable Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 2. Create Enums
CREATE TYPE checkpoint_category AS ENUM (
  'academic',
  'party',
  'sports',
  'start',
  'afterparty',
  'default'
);

CREATE TYPE user_role AS ENUM (
  'admin',
  'checkpoint',
  'team'
);

-- 3. Auto-update Trigger Function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- CORE ENTITIES
-- ============================================================================

-- Areas
CREATE TABLE areas (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Checkpoints
CREATE TABLE checkpoints (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  area_id UUID REFERENCES areas(id) ON DELETE SET NULL,
  number INTEGER,
  name TEXT NOT NULL,
  category checkpoint_category NOT NULL DEFAULT 'default',
  location_name TEXT,
  latitude DOUBLE PRECISION NOT NULL DEFAULT 0.0,
  longitude DOUBLE PRECISION NOT NULL DEFAULT 0.0,
  accessible BOOLEAN NOT NULL DEFAULT true,
  lanes INTEGER NOT NULL DEFAULT 1,
  checkpoint_description JSONB,
  org_description JSONB,
  requirements TEXT,
  execution TEXT,
  url TEXT,
  contact_person TEXT,
  contact_email TEXT,
  contact_phone TEXT,
  cancelled BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Teams
CREATE TABLE teams (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  number INTEGER UNIQUE,
  participants INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Users (Unified table for Admins, Checkpoint Staff, and Teams)
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  role user_role NOT NULL DEFAULT 'team',
  
  -- Identifier fields (Email for admins/checkpoints, Phone/Code/Identifier optional for teams)
  email TEXT UNIQUE,
  phone TEXT,
  name TEXT,

  -- Foreign Key references based on role
  checkpoint_id UUID REFERENCES checkpoints(id) ON DELETE SET NULL,
  team_id UUID REFERENCES teams(id) ON DELETE SET NULL,

  -- OTP Authentication Fields
  otp_code_hash TEXT,
  otp_expires_at TIMESTAMPTZ,
  otp_sent_at TIMESTAMPTZ,
  otp_attempts INTEGER NOT NULL DEFAULT 0,
  last_login_at TIMESTAMPTZ,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  -- Ensure admins/checkpoints have emails and role linkages match entity references
  CONSTRAINT check_role_references CHECK (
    (role = 'admin') OR
    (role = 'checkpoint' AND checkpoint_id IS NOT NULL) OR
    (role = 'team')
  )
);

-- Scores
CREATE TABLE scores (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  checkpoint_id UUID NOT NULL REFERENCES checkpoints(id) ON DELETE CASCADE,
  recorded_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
  score INTEGER NOT NULL CONSTRAINT score_range CHECK (score >= 0 AND score <= 12),
  participants_present INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,
  
  CONSTRAINT unique_team_checkpoint_score UNIQUE (team_id, checkpoint_id)
);

-- ============================================================================
-- MEDIA, VOTES & SUGGESTIONS
-- ============================================================================

-- Photos
CREATE TABLE photos (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  s3_key TEXT NOT NULL,
  url TEXT NOT NULL,
  team_id UUID REFERENCES teams(id) ON DELETE SET NULL,
  checkpoint_id UUID REFERENCES checkpoints(id) ON DELETE SET NULL,
  published BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Votes
CREATE TABLE votes (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
  voter_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  
  CONSTRAINT unique_photo_voter UNIQUE (photo_id, voter_hash)
);

-- Photo Team Suggestions
CREATE TABLE photo_team_suggestions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  photo_id UUID NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
  suggested_team_number INTEGER NOT NULL,
  voter_hash TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- RATINGS, REPORTS & NEWS
-- ============================================================================

-- Checkpoint Ratings
CREATE TABLE checkpoint_ratings (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  checkpoint_id UUID NOT NULL REFERENCES checkpoints(id) ON DELETE CASCADE,
  team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
  rating INTEGER NOT NULL CONSTRAINT check_rating_range CHECK (rating >= 0 AND rating <= 5),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Team Ratings
CREATE TABLE team_ratings (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  checkpoint_id UUID NOT NULL REFERENCES checkpoints(id) ON DELETE CASCADE,
  rating INTEGER NOT NULL CONSTRAINT team_rating_range CHECK (rating >= 0 AND rating <= 5),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  CONSTRAINT unique_checkpoint_team_rating UNIQUE (checkpoint_id, team_id)
);

-- Checkpoint Reports
CREATE TABLE checkpoint_reports (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  checkpoint_id UUID NOT NULL REFERENCES checkpoints(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  content JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- Team Reports
CREATE TABLE team_reports (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  checkpoint_id UUID REFERENCES checkpoints(id) ON DELETE SET NULL,
  title TEXT NOT NULL,
  content JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- News
CREATE TABLE news (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  title TEXT NOT NULL,
  content JSONB NOT NULL,
  published_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Fast user lookups during OTP generation and verification
CREATE INDEX idx_users_email ON users(email) WHERE email IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX idx_users_role ON users(role) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_checkpoint ON users(checkpoint_id) WHERE checkpoint_id IS NOT NULL;
CREATE INDEX idx_users_team ON users(team_id) WHERE team_id IS NOT NULL;

-- Core indexes
CREATE INDEX idx_checkpoints_area ON checkpoints(area_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_scores_team ON scores(team_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_scores_checkpoint ON scores(checkpoint_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_photos_published ON photos(published) WHERE deleted_at IS NULL;

-- ============================================================================
-- TRIGGERS
-- ============================================================================

CREATE TRIGGER update_areas_updated_at BEFORE UPDATE ON areas FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_checkpoints_updated_at BEFORE UPDATE ON checkpoints FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_teams_updated_at BEFORE UPDATE ON teams FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_scores_updated_at BEFORE UPDATE ON scores FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_photos_updated_at BEFORE UPDATE ON photos FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_checkpoint_ratings_updated_at BEFORE UPDATE ON checkpoint_ratings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_team_ratings_updated_at BEFORE UPDATE ON team_ratings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_checkpoint_reports_updated_at BEFORE UPDATE ON checkpoint_reports FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_team_reports_updated_at BEFORE UPDATE ON team_reports FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_news_updated_at BEFORE UPDATE ON news FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
