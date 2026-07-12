# =============================================================================
# Tardigrade Git Module UI - Dockerfile
# =============================================================================
#
# Multi-stage build pour optimiser la taille de l'image finale.
#
# Stage 1 : Build de l'application avec Vite
# Stage 2 : Image finale avec Nginx
#
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1 : Build
# -----------------------------------------------------------------------------
FROM node:18-alpine AS builder

# Installer pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

# Créer le répertoire de travail
WORKDIR /app

# Copier les fichiers de configuration
COPY ui/git/package*.json ./
COPY ui/git/pnpm-lock.yaml ./

# Installer les dépendances
RUN pnpm install --frozen-lockfile

# Copier le reste des fichiers
COPY ui/git/ .

# Builder l'application
RUN pnpm run build

# -----------------------------------------------------------------------------
# Stage 2 : Runtime
# -----------------------------------------------------------------------------
FROM nginx:alpine

# Copier la configuration Nginx
COPY ui/git/nginx.conf /etc/nginx/conf.d/default.conf

# Copier les fichiers de build
COPY --from=builder /app/dist /usr/share/nginx/html

# Exposer le port
EXPOSE 80

# Configuration du healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

# Point d'entrée
CMD ["nginx", "-g", "daemon off;"]
