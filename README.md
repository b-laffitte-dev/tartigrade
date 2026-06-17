# 🦕 Tardigrade-CI

> *Une plateforme DevOps modulaire, open-source, conçue pour survivre à tous les environnements.*

---

## 🚀 À propos

**Tardigrade-CI** est une plateforme web modulaire dans l'esprit de GitHub/GitLab, mais **100% open-source et extensible**. Elle combine :

- ✅ **Gestionnaire de code source (Git)** - Repository Git auto-hébergé
- ✅ **Intégration Continue (CI)** - Pipelines YAML multi-langages
- ✅ **Entrepôt de binaires** - Stockage multi-format (Docker, npm, cargo, etc.)
- ✅ **Système de plugins** - Architecture modulaire pour étendre les fonctionnalités

**Pourquoi Tardigrade-CI ?**
- 🔓 **Pas de vendor lock-in** : Choisissez vos outils, pas ceux imposés par une plateforme
- 🚀 **Léger et modulaire** : Activez/désactivez des composants à la demande
- 🏠 **Auto-hébergement** : Contrôle total sur vos données
- 💰 **Économique** : Évitez les coûts élevés de GitHub/GitLab Enterprise
- 🔧 **Extensible** : Intégrez vos outils préférés via plugins

---

## 📋 Plan de Projet

Le plan détaillé du projet est disponible ici :
- **[TARDIGRADE-CI-PLAN.md](TARDIGRADE-CI-PLAN.md)** *(Plan complet - 40+ KB)*
- **[.vibe/project_plan.md](.vibe/project_plan.md)** *(Résumé et index)*

### 🎯 Statut Actuel
- **Phase:** Discovery (0-4 semaines)
- **Prochaine étape:** Atelier de cadrage technique (20/06/2026)
- **Lancement MVP prévu:** Décembre 2026

---

## 📚 Documentation

*(À créer - Structure prévue)*

| Section | Description | Lien |
|---------|-------------|------|
| **Guides de démarrage** | Installation, configuration | `[À créer]` |
| **API Reference** | Documentation de l'API REST/GraphQL | `[À créer]` |
| **Plugins** | Développement de plugins | `[À créer]` |
| **Contribution** | Comment contribuer au projet | `[CONTRIBUTING.md](CONTRIBUTING.md)` *(À créer)* |
| **Architecture** | Design technique détaillé | `[ARCHITECTURE.md](ARCHITECTURE.md)` *(À créer)* |

---

## 🛠️ Stack Technique

| Composant | Technologie |
|-----------|-------------|
| **Frontend** | React + TypeScript + TailwindCSS |
| **Backend** | Rust (Axum) + Go |
| **Base de données** | PostgreSQL + Redis |
| **Stockage** | MinIO (S3-compatible) |
| **CI Workers** | Rust (pour les builds rapides) |
| **Containerisation** | Docker + Kubernetes |
| **Infrastructure** | Terraform + Ansible |

---

## 📦 Modules Principaux

### 1. Git Module
- Gestion de repositories Git
- Branches, Pull Requests, Issues
- Webhooks et intégrations

### 2. CI Module
- Pipelines YAML
- Workers éphémères
- Support multi-langages (Rust, Go, Python, JS/TS)

### 3. Artifact Registry
- Stockage de conteneurs (Docker/OCI)
- Gestion de packages (npm, cargo, Maven, etc.)
- Versionnage des artefacts

### 4. Plugin System
- Architecture modulaire
- Sandboxing pour sécurité
- Marketplace de plugins (futur)

---

## 🚀 Roadmap

### MVP (v0.1.0 - Décembre 2026)
- [ ] Git Module (repositories, branches, PR)
- [ ] CI Module (pipelines basiques)
- [ ] Artifact Registry (Docker, npm, cargo)
- [ ] API REST/GraphQL
- [ ] Plugin System (basique)

### V1.0 (2027)
- [ ] RBAC avancé
- [ ] UI améliorée (Dashboard, Analytics)
- [ ] Multi-region / Geo-redundancy
- [ ] Marketplace de plugins
- [ ] Intégrations natives (Jira, Slack, etc.)

**Voir la roadmap complète:** [TARDIGRADE-CI-PLAN.md#5-roadmap--planning](TARDIGRADE-CI-PLAN.md#5-roadmap--planning)

---

## 🤝 Contribution

Nous serons ravis de votre contribution ! 

*(À créer - Structure prévue)*

1. Fork le projet
2. Créez une branche (`git checkout -b feature/amazing-feature`)
3. Committez vos changements (`git commit -m 'Add amazing feature'`)
4. Poussez vers la branche (`git push origin feature/amazing-feature`)
5. Ouvrez une Pull Request

**Voir:** [CONTRIBUTING.md](CONTRIBUTING.md) *(À créer)*

---

## 📊 Métriques & Statistiques

*(À créer - Exemple de métriques prévues)*

- ✅ Étoiles GitHub: [![Stars](https://img.shields.io/github/stars/tardigrade-ci/tardigrade?style=social)](https://github.com/tardigrade-ci/tardigrade)
- ✅ Téléchargements Docker: [![Docker Pulls](https://img.shields.io/docker/pulls/tardigradeci/tardigrade)](https://hub.docker.com/r/tardigradeci/tardigrade)
- ✅ Instances déployées: *(À mettre en place)*
- ✅ Uptime: [![Uptime](https://img.shields.io/uptimerobot/status/m782123456)](https://status.tardigrade-ci.dev) *(À créer)*

---

## 📞 Communauté

*(À créer)*

- **Discord:** [Rejoindre le serveur](https://discord.gg/tardigrade-ci) *(À créer)*
- **Twitter:** [@TardigradeCI](https://twitter.com/TardigradeCI) *(À créer)*
- **Email:** contact@tardigrade-ci.dev *(À créer)*

---

## 📄 License

*(À décider - Options: AGPL-3.0, MIT, Apache-2.0)*

---

## 🙏 Remerciements

- Inspiré par: [Gitea](https://gitea.io/), [Woodpecker CI](https://woodpecker-ci.org/), [Harbor](https://goharbor.io/)
- Merci à la communauté open-source DevOps pour son inspiration

---

**© 2026 Tardigrade-CI**

*Made with ❤️ by DevOps, for DevOps.*
