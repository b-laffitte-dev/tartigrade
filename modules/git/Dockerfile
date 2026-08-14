# =============================================================================
# Tardigrade Git Module - Dockerfile
# =============================================================================
# 
# Multi-stage build pour optimiser la taille de l'image finale.
# 
# Stage 1 : Build de l'application Rust
# Stage 2 : Image finale avec uniquement le binaire
#
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1 : Build
# -----------------------------------------------------------------------------
FROM rust:1.97-slim AS builder

# Installer les dépendances nécessaires pour le build.
# Avec sqlx en mode rustls, aucune bibliothèque SSL native n'est requise.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Créer le répertoire de travail
WORKDIR /usr/src/tardigrade

# Créer la structure de répertoires
RUN mkdir -p modules/git/src && mkdir -p crates

# Copier les fichiers de dépendances d'abord pour profiter du cache
COPY Cargo.toml Cargo.toml
COPY modules/git/Cargo.toml modules/git/Cargo.toml
COPY crates crates

# Créer un utilisateur pour le build (meilleure sécurité)
RUN useradd -m tardigrade && \
    chown -R tardigrade:tardigrade /usr/src/tardigrade

# Copier le code source
USER tardigrade
COPY modules/git/src modules/git/src

# Builder l'application en release mode
RUN cargo build --release --bin tardigrade-git

# -----------------------------------------------------------------------------
# Stage 2 : Runtime
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim

# Installer les dépendances minimales pour le runtime.
# rustls évite toute dépendance native à OpenSSL ; curl reste pour le healthcheck.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Créer le répertoire de travail
WORKDIR /app

# Créer un utilisateur non-root pour exécuter l'application
RUN useradd -m tardigrade

# Copier le binaire depuis le stage builder
COPY --from=builder /usr/src/tardigrade/target/release/tardigrade-git /app/tardigrade-git

# Changer les permissions du binaire
RUN chmod +x /app/tardigrade-git && \
    chown tardigrade:tardigrade /app/tardigrade-git

# Basculer sur l'utilisateur non-root
USER tardigrade

# Exposer le port
EXPOSE 3001

# Configuration du healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/api/health || exit 1

# Point d'entrée
ENTRYPOINT ["/app/tardigrade-git"]
