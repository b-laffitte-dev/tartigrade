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

# Définir les arguments de build avec des valeurs par défaut
ARG VITE_API_BASE_URL=/api
ARG VITE_GIT_MODULE_URL=/api

# Installer pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

# Créer le répertoire de travail
WORKDIR /app

# Copier les fichiers de configuration (chemin relatif depuis le contexte racine)
COPY ui/git/package*.json ./
COPY ui/git/package-lock.json ./

# Installer les dépendances
RUN npm install --frozen-lockfile

# Copier le reste des fichiers du module ui/git
COPY ui/git/ .

# Builder l'application avec les variables d'environnement
RUN VITE_API_BASE_URL=${VITE_API_BASE_URL} VITE_GIT_MODULE_URL=${VITE_GIT_MODULE_URL} npm run build

# -----------------------------------------------------------------------------
# Stage 2 : Runtime
# -----------------------------------------------------------------------------
FROM nginx:alpine

# Installer wget pour le healthcheck
RUN apk add --no-cache wget

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
