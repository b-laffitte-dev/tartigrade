# =============================================================================
# Tardigrade-CI - Makefile
# =============================================================================
#
# Ce Makefile automatise les tâches de build, test et déploiement du projet.
#
# Usage:
#   make help              - Affiche cette aide
#   make env-check         - Vérifie que toutes les dépendances sont installées
#   make deps              - Installe toutes les dépendances (Rust + Node)
#
#   make build             - Build complet du projet (backend + frontend)
#   make build-backend     - Build du backend Rust
#   make build-frontend    - Build du frontend
#
#   make run               - Lance tous les services avec Docker Compose
#   make run-dev           - Lance backend + DB en mode dev
#   make down              - Arrête tous les conteneurs
#   make restart           - Redémarre tous les services
#
#   make dev-backend       - Lance le backend en mode dev (cargo run)
#   make dev-frontend      - Lance le frontend en mode dev (Vite)
#
#   make test              - Exécute tous les tests
#   make test-backend      - Tests backend uniquement
#   make test-frontend     - Tests frontend uniquement
#
#   make clean             - Nettoie les artefacts de build
#   make clean-all         - Nettoie tout (y compris node_modules et Cargo.lock)
#
#   make logs              - Affiche les logs de tous les services
#   make ps                - Liste les conteneurs en cours d'exécution
#   make health            - Vérifie la santé des services
#
# =============================================================================

# =============================================================================
# Configuration
# =============================================================================

# Chemins
PROJECT_ROOT := $(shell pwd)
BACKEND_DIR := $(PROJECT_ROOT)/modules/git
FRONTEND_DIR := $(PROJECT_ROOT)/ui/git
DOCKER_DIR := $(PROJECT_ROOT)/docker

# Noms des services Docker
DOCKER_COMPOSE := docker compose
DOCKER_COMPOSE_FILE := $(DOCKER_DIR)/docker-compose.yml

# Ports
BACKEND_PORT := 3001
FRONTEND_DEV_PORT := 5173
FRONTEND_PROD_PORT := 8080
POSTGRES_PORT := 5432

# =============================================================================
# Cibles principales
# =============================================================================

.PHONY: help
help:
	@echo "Tardigrade-CI - Makefile"
	@echo ""
	@echo "Cibles disponibles:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'
	@echo ""
	@echo "Environnement:"
	@echo "  BACKEND_DIR:   $(BACKEND_DIR)"
	@echo "  FRONTEND_DIR:  $(FRONTEND_DIR)"
	@echo "  DOCKER_DIR:    $(DOCKER_DIR)"

.PHONY: env-check
env-check: ## Verifie que toutes les dependances sont installees
	@echo "==========================================="
	@echo "Verification des dependances..."
	@echo "==========================================="
	@echo ""
	
	# Verification de Rust
	@if command -v cargo >/dev/null 2>&1; then \
		echo "[OK] Rust/Cargo: $$(cargo --version | head -n1)"; \
	else \
		echo "[ERREUR] Rust/Cargo: NON INSTALLE"; \
		echo "  -> Installez Rust avec: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"; \
		exit 1; \
	fi
	
	# Verification de Node.js
	@if command -v node >/dev/null 2>&1; then \
		echo "[OK] Node.js: $$(node --version)"; \
	else \
		echo "[ERREUR] Node.js: NON INSTALLE"; \
		echo "  -> Installez Node.js depuis https://nodejs.org/"; \
		exit 1; \
	fi
	
	# Verification de npm
	@if command -v npm >/dev/null 2>&1; then \
		echo "[OK] npm: $$(npm --version)"; \
	else \
		echo "[ERREUR] npm: NON INSTALLE"; \
		echo "  -> npm est normalement installe avec Node.js"; \
		exit 1; \
	fi
	
	# Verification de Docker
	@if command -v docker >/dev/null 2>&1; then \
		echo "[OK] Docker: $$(docker --version | head -n1)"; \
		if docker compose version >/dev/null 2>&1; then \
			echo "[OK] Docker Compose: disponible"; \
		else \
			echo "[AVERTISSEMENT] Docker Compose: utilisez 'docker compose' (integre a Docker)"; \
		fi; \
	else \
		echo "[ERREUR] Docker: NON INSTALLE"; \
		echo "  -> Installez Docker depuis https://www.docker.com/"; \
		exit 1; \
	fi
	
	# Verification de Git
	@if command -v git >/dev/null 2>&1; then \
		echo "[OK] Git: $$(git --version | head -n1)"; \
	else \
		echo "[ERREUR] Git: NON INSTALLE"; \
		echo "  -> Installez Git depuis https://git-scm.com/"; \
		exit 1; \
	fi
	
	@echo ""
	@echo "==========================================="
	@echo "Toutes les dependances sont installees!"
	@echo "==========================================="

.PHONY: deps
deps: ## Installe toutes les dependances (Rust toolchain + Node modules)
	@echo "==========================================="
	@echo "Installation des dependances..."
	@echo "==========================================="
	@echo ""
	
	@echo "[INFO] Mise a jour de la toolchain Rust..."
	@rustup update 2>/dev/null || echo "[AVERTISSEMENT] rustup update a echoue ou rustup n'est pas installe"
	
	@echo "[INFO] Verification du workspace Rust..."
	@cd $(PROJECT_ROOT) && cargo check --workspace 2>&1 | head -20 || echo "[AVERTISSEMENT] cargo check a echoue"
	
	@echo "[INFO] Installation des dependances Node.js..."
	@cd $(FRONTEND_DIR) && npm install
	
	@echo ""
	@echo "==========================================="
	@echo "Dependances installees!"
	@echo "==========================================="

.PHONY: build
build: build-backend build-frontend ## Build complet du projet

.PHONY: build-backend
build-backend: ## Build du backend Rust
	@echo "[INFO] Build du backend Rust..."
	@cd $(BACKEND_DIR) && cargo build --release
	@echo "[OK] Backend built avec succes!"

.PHONY: build-frontend
build-frontend: ## Build du frontend
	@echo "[INFO] Build du frontend..."
	@cd $(FRONTEND_DIR) && npm run build
	@echo "[OK] Frontend built avec succes!"

.PHONY: run
run: ## Lance tous les services avec Docker Compose
	@echo "[INFO] Lancement de tous les services avec Docker Compose..."
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up -d
	@sleep 5
	@echo "[OK] Services lances!"
	@echo ""
	@echo "URLs:"
	@echo "  Backend:   http://localhost:$(BACKEND_PORT)"
	@echo "  Frontend:  http://localhost:$(FRONTEND_PROD_PORT)"
	@echo "  PostgreSQL: localhost:$(POSTGRES_PORT)"

.PHONY: run-dev
run-dev: ## Lance backend + DB en mode dev (sans frontend container)
	@echo "[INFO] Lancement de PostgreSQL et du backend en mode developpement..."
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up -d postgres
	@sleep 3
	@cd $(BACKEND_DIR) && cargo run

.PHONY: down
down: ## Arrete tous les conteneurs
	@echo "[INFO] Arret des conteneurs..."
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down
	@echo "[OK] Conteneurs arretes!"

.PHONY: restart
restart: down run ## Redemarre tous les services

.PHONY: dev-backend
dev-backend: ## Lance le backend en mode dev (cargo run)
	@echo "[INFO] Lancement du backend en mode developpement..."
	@echo "[INFO] Pour arreter: Ctrl+C"
	@cd $(BACKEND_DIR) && cargo run

.PHONY: dev-frontend
dev-frontend: ## Lance le frontend en mode dev (Vite)
	@echo "[INFO] Lancement du frontend en mode developpement (Vite)..."
	@echo "[INFO] Pour arreter: Ctrl+C"
	@cd $(FRONTEND_DIR) && npm run dev

.PHONY: test
test: test-backend test-frontend ## Execute tous les tests

.PHONY: test-backend
test-backend: ## Tests backend uniquement
	@echo "[INFO] Execution des tests backend..."
	@cd $(BACKEND_DIR) && cargo test --workspace
	@echo "[OK] Tests backend termines!"

.PHONY: test-frontend
test-frontend: ## Tests frontend uniquement
	@echo "[INFO] Execution des tests frontend..."
	@cd $(FRONTEND_DIR) && npm test
	@echo "[OK] Tests frontend termines!"

.PHONY: clean
clean: ## Nettoie les artefacts de build
	@echo "[INFO] Nettoyage des artefacts de build..."
	
	# Backend
	@cd $(BACKEND_DIR) && cargo clean 2>/dev/null || echo "[AVERTISSEMENT] cargo clean a echoue"
	
	# Frontend
	@cd $(FRONTEND_DIR) && rm -rf dist/ .vite/ 2>/dev/null || echo "[AVERTISSEMENT] Suppression des artefacts frontend a echoue"
	
	# Docker
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down -v 2>/dev/null || echo "[AVERTISSEMENT] docker compose down a echoue"
	
	@echo "[OK] Nettoyage termine!"

.PHONY: clean-all
clean-all: clean ## Nettoie tout (y compris node_modules et Cargo.lock)
	@echo "[INFO] Nettoyage complet..."
	
	# Suppression des node_modules
	@find $(PROJECT_ROOT) -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null || echo "[AVERTISSEMENT] Suppression de node_modules a echoue"
	
	# Suppression des Cargo.lock
	@find $(PROJECT_ROOT) -name "Cargo.lock" -type f -delete 2>/dev/null || echo "[AVERTISSEMENT] Suppression de Cargo.lock a echoue"
	
	# Suppression des caches
	@rm -rf $(PROJECT_ROOT)/target/ 2>/dev/null || echo "[AVERTISSEMENT] Suppression de target/ a echoue"
	@rm -rf $(PROJECT_ROOT)/.cargo/ 2>/dev/null || echo "[AVERTISSEMENT] Suppression de .cargo/ a echoue"
	
	@echo "[OK] Nettoyage complet termine!"

.PHONY: docker-build
docker-build: ## Build les images Docker
	@echo "[INFO] Build des images Docker..."
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) build
	@echo "[OK] Images Docker buildes!"

.PHONY: docker-push
docker-push: ## Push les images Docker (necessite d'etre logue)
	@echo "[INFO] Push des images Docker..."
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) push
	@echo "[OK] Images Docker poussees!"

.PHONY: logs
logs: ## Affiche les logs de tous les services
	@echo "[INFO] Affichage des logs de tous les services... (Ctrl+C pour arreter)"
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) logs -f

.PHONY: ps
ps: ## Liste les conteneurs en cours d'execution
	@echo "[INFO] Conteneurs en cours d'execution:"
	@cd $(PROJECT_ROOT) && $(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) ps

.PHONY: health
health: ## Verifie la sante des services
	@echo "==========================================="
	@echo "Verification de la sante des services..."
	@echo "==========================================="
	@echo ""
	
	# Verification du backend
	@echo -n "Backend (port $(BACKEND_PORT)): "
	@if curl -s -o /dev/null -w "%{http_code}" http://localhost:$(BACKEND_PORT)/api/health 2>/dev/null | grep -q "200"; then \
		echo "[OK]"; \
	else \
		echo "[ERREUR] - non accessible"; \
	fi
	
	# Verification de la base de donnees
	@echo -n "PostgreSQL (port $(POSTGRES_PORT)): "
	@if docker inspect tardigrade-postgres >/dev/null 2>&1; then \
		if docker exec tardigrade-postgres pg_isready -U postgres -q 2>/dev/null; then \
			echo "[OK]"; \
		else \
			echo "[ERREUR] - non pret"; \
		fi; \
	else \
		echo "[ERREUR] - conteneur non demarre"; \
	fi
	
	@echo ""
	@echo "Pour demarrer les services: make run"
	@echo "==========================================="

.PHONY: setup-db
setup-db: ## Initialise la base de donnees (migrations)
	@echo "[INFO] Initialisation de la base de donnees..."
	@cd $(BACKEND_DIR) && sqlx database setup 2>/dev/null || echo "[AVERTISSEMENT] sqlx database setup a echoue"
	@cd $(BACKEND_DIR) && sqlx migrate run 2>/dev/null || echo "[AVERTISSEMENT] sqlx migrate run a echoue"
	@echo "[OK] Base de donnees initialisee!"

.PHONY: migrate-db
migrate-db: ## Applique les migrations de la base de donnees
	@echo "[INFO] Application des migrations..."
	@cd $(BACKEND_DIR) && sqlx migrate run 2>/dev/null || echo "[AVERTISSEMENT] sqlx migrate run a echoue"
	@echo "[OK] Migrations appliquees!"

.PHONY: lint
lint: lint-backend lint-frontend ## Execute le linting sur tout le projet

.PHONY: lint-backend
lint-backend: ## Linting du backend Rust
	@echo "[INFO] Linting du backend Rust..."
	@cd $(BACKEND_DIR) && cargo clippy --workspace -- -D warnings 2>/dev/null || echo "[AVERTISSEMENT] cargo clippy a echoue"
	@echo "[OK] Linting backend termine!"

.PHONY: lint-frontend
lint-frontend: ## Linting du frontend
	@echo "[INFO] Linting du frontend..."
	@cd $(FRONTEND_DIR) && npm run lint 2>/dev/null || echo "[AVERTISSEMENT] npm run lint a echoue"
	@echo "[OK] Linting frontend termine!"

.PHONY: format
format: format-backend format-frontend ## Formate tout le code

.PHONY: format-backend
format-backend: ## Formate le code Rust
	@echo "[INFO] Formatage du code Rust..."
	@cd $(PROJECT_ROOT) && cargo fmt --workspace 2>/dev/null || echo "[AVERTISSEMENT] cargo fmt a echoue"
	@echo "[OK] Code Rust formate!"

.PHONY: format-frontend
format-frontend: ## Formate le code frontend
	@echo "[INFO] Formatage du code frontend..."
	@cd $(FRONTEND_DIR) && npm run format 2>/dev/null || echo "[AVERTISSEMENT] npm run format a echoue"
	@echo "[OK] Code frontend formate!"

# =============================================================================
# Cibles pour le developpement rapide
# =============================================================================

.PHONY: quick-start
quick-start: env-check deps build run health ## Demarrage rapide complet

.PHONY: reset
reset: clean-all deps build run ## Reinitialisation complete du projet

# =============================================================================
# Configuration par défaut
# =============================================================================

# Utilise docker compose V2 par défaut
DOCKER_COMPOSE := docker compose

# Fin du Makefile
# =============================================================================
