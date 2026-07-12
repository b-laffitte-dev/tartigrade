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
FROM rust:1.70-slim AS builder

# Installer les dépendances nécessaires pour le build
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Créer le répertoire de travail
WORKDIR /usr/src/tardigrade

# Copier les fichiers de dépendances d'abord pour profiter du cache
COPY modules/git/Cargo.toml modules/git/Cargo.toml
COPY Cargo.toml Cargo.toml
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
FROM debian:bullseye-slim

# Installer les dépendances minimales pour le runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl1.1 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Créer le répertoire de travail
WORKDIR /usr/local/bin

# Copier le binaire depuis le stage builder
COPY --from=builder /usr/src/tardigrade/target/release/tardigrade-git /usr/local/bin/tardigrade-git

# Créer un utilisateur non-root pour exécuter l'application
RUN useradd -m tardigrade && \
    chown tardigrade:tardigrade /usr/local/bin/tardigrade-git

# Basculer sur l'utilisateur non-root
USER tardigrade

# Exposer le port
EXPOSE 3001

# Configuration du healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/api/health || exit 1

# Point d'entrée
ENTRYPOINT ["/usr/local/bin/tardigrade-git"]
