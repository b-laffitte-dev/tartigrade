-- Migration : trigger automatique pour `updated_at`
--
-- Met à jour automatiquement la colonne `updated_at` lors d'un UPDATE
-- sur la table `repositories`, afin que l'horodatage reste cohérent
-- sans dépendre de la logique applicative.

-- Fonction réutilisable de mise à jour de l'horodatage
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger sur repositories
DROP TRIGGER IF EXISTS trg_repositories_updated_at ON repositories;
CREATE TRIGGER trg_repositories_updated_at
    BEFORE UPDATE ON repositories
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
