# 🚀 Deployment Guide - Tardigrade-CI

**Version :** 1.0  
**Last Updated :** 2026-06-19  
**Status :** Draft  
**Author :** Benzo + Mistral Vibe  

---

## 📋 Table of Contents
1. [Deployment Overview](#1-deployment-overview)
2. [Prerequisites](#2-prerequisites)
3. [Docker Compose Deployment](#4-docker-compose-deployment-development)
4. [Kubernetes Deployment](#5-kubernetes-deployment-production)
5. [Configuration](#7-configuration)
6. [Monitoring](#12-monitoring--logging)
7. [Backup & Recovery](#13-backup--recovery)

---

## 1️⃣ Deployment Overview

### Supported Methods
| Method | Use Case | Complexity |
|--------|----------|------------|
| **Docker Compose** | Local development | ⭐ |
| **Kubernetes** | Production | ⭐⭐⭐ |
| **Helm** | Production with customization | ⭐⭐ |

### Architecture
```mermaid
graph TD
    A[API Gateway] --> B[Git Module]
    A --> C[CI Module]
    A --> D[Registry Module]
    B --> E[PostgreSQL Git]
    B --> F[Redis Git]
    C --> G[PostgreSQL CI]
    C --> H[Redis CI]
    C --> I[NATS]
    D --> J[PostgreSQL Registry]
    D --> K[Redis Registry]
    D --> L[MinIO]
```

### Requirements
| Resource | Minimum | Recommended |
|----------|---------|-------------|
| CPU | 4 vCPUs | 8+ vCPUs |
| RAM | 8 GB | 16+ GB |
| Disk | 100 GB | 500+ GB SSD |

---

## 2️⃣ Prerequisites

### Docker
```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker
```

### Docker Compose
```bash
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### Kubernetes Tools
```bash
# kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# Helm
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

# Minikube (dev)
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube
```

---

## 4️⃣ Docker Compose Deployment (Development)

### Quick Start
```bash
git clone https://github.com/tardigrade-ci/tardigrade.git
cd tardigrade
cp .env.example .env
nano .env  # Edit configuration
docker-compose up -d
```

### .env Example
```bash
HOST_HTTP_PORT=8080
HOST_GRPC_PORT=50051
HOST_FRONTEND_PORT=3000
POSTGRES_USER=tardigrade
POSTGRES_PASSWORD=your_password
MINIO_ROOT_USER=minioadmin
MINIO_ROOT_PASSWORD=your_minio_password
JWT_SECRET=your_jwt_secret
SESSION_SECRET=your_session_secret
RUST_LOG=info
```

### Docker Compose File
```yaml
version: '3.8'
services:
  api-gateway:
    image: tardigradeci/api-gateway:latest
    ports:
      - "8080:8080"
      - "50051:50051"
    environment:
      - DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres-git:5432/tardigrade_git
      - REDIS_URL=redis://redis-git:6379
      - NATS_URL=nats://nats:4222
      - MINIO_ENDPOINT=minio:9000
      - JWT_SECRET=${JWT_SECRET}
    depends_on:
      - postgres-git
      - redis-git
      - nats
      - minio

  postgres-git:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
      - POSTGRES_DB=tardigrade_git
    volumes:
      - ./data/postgres-git:/var/lib/postgresql/data

  redis-git:
    image: redis:7-alpine
    command: redis-server --appendonly yes
    volumes:
      - ./data/redis-git:/data

  nats:
    image: nats:2.10-alpine
    command: --jetstream --store_dir=/data
    volumes:
      - ./data/nats:/data

  minio:
    image: minio/minio:latest
    command: server /data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=${MINIO_ROOT_USER}
      - MINIO_ROOT_PASSWORD=${MINIO_ROOT_PASSWORD}
    volumes:
      - ./data/minio:/data

  frontend:
    image: tardigradeci/frontend:latest
    ports:
      - "3000:3000"
    environment:
      - VITE_API_URL=http://localhost:8080
```

### Useful Commands
```bash
docker-compose logs -f api-gateway
docker-compose exec api-gateway sh
docker-compose down -v  # WARNING: deletes all data
```

---

## 5️⃣ Kubernetes Deployment (Production)

### Namespace
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: tardigrade-ci
```

### PostgreSQL (Git)
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres-git
spec:
  serviceName: postgres-git
  replicas: 1
  selector:
    matchLabels:
      app: postgres-git
  template:
    metadata:
      labels:
        app: postgres-git
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        env:
        - name: POSTGRES_USER
          value: tardigrade
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: postgres-secrets
              key: password
        - name: POSTGRES_DB
          value: tardigrade_git
        volumeMounts:
        - name: data
          mountPath: /var/lib/postgresql/data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ReadWriteOnce]
      storageClassName: standard
      resources:
        requests:
          storage: 10Gi
---
apiVersion: v1
kind: Service
metadata:
  name: postgres-git
spec:
  selector:
    app: postgres-git
  ports:
  - port: 5432
```

### Redis (Git)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis-git
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis-git
  template:
    metadata:
      labels:
        app: redis-git
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        command: [redis-server, --appendonly, yes]
        volumeMounts:
        - name: data
          mountPath: /data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: redis-git-pvc
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: redis-git-pvc
spec:
  accessModes:
  - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
---
apiVersion: v1
kind: Service
metadata:
  name: redis-git
spec:
  selector:
    app: redis-git
  ports:
  - port: 6379
```

### NATS with JetStream
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: nats
spec:
  serviceName: nats
  replicas: 3
  selector:
    matchLabels:
      app: nats
  template:
    metadata:
      labels:
        app: nats
    spec:
      containers:
      - name: nats
        image: nats:2.10-alpine
        command:
        - /nats-server
        - --jetstream
        - --store_dir=/data
        - --cluster_name=tardigrade-nats
        - --cluster=nats://0.0.0.0:6222
        - --routes=nats://nats-0.nats:6222,nats://nats-1.nats:6222,nats://nats-2.nats:6222
        ports:
        - containerPort: 4222
        - containerPort: 6222
        - containerPort: 8222
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ReadWriteOnce]
      resources:
        requests:
          storage: 10Gi
---
apiVersion: v1
kind: Service
metadata:
  name: nats
spec:
  selector:
    app: nats
  ports:
  - port: 4222
    name: client
  - port: 6222
    name: cluster
  - port: 8222
    name: monitoring
  clusterIP: None
---
apiVersion: v1
kind: Service
metadata:
  name: nats-external
spec:
  selector:
    app: nats
  ports:
  - port: 4222
  type: LoadBalancer
```

### MinIO
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: minio
spec:
  serviceName: minio
  replicas: 4
  selector:
    matchLabels:
      app: minio
  template:
    metadata:
      labels:
        app: minio
    spec:
      containers:
      - name: minio
        image: minio/minio:latest
        args:
        - server
        - /data
        - --console-address
        - :9001
        env:
        - name: MINIO_ROOT_USER
          valueFrom:
            secretKeyRef:
              name: minio-secrets
              key: root-user
        - name: MINIO_ROOT_PASSWORD
          valueFrom:
            secretKeyRef:
              name: minio-secrets
              key: root-password
        ports:
        - containerPort: 9000
        - containerPort: 9001
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ReadWriteOnce]
      resources:
        requests:
          storage: 100Gi
---
apiVersion: v1
kind: Service
metadata:
  name: minio
spec:
  selector:
    app: minio
  ports:
  - port: 9000
    name: api
  - port: 9001
    name: console
  type: ClusterIP
---
apiVersion: v1
kind: Service
metadata:
  name: minio-external
spec:
  selector:
    app: minio
  ports:
  - port: 9000
    name: api
  - port: 9001
    name: console
  type: LoadBalancer
```

### API Gateway
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
spec:
  replicas: 2
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: api-gateway
        image: tardigradeci/api-gateway:1.0.0
        ports:
        - containerPort: 8080
        - containerPort: 50051
        envFrom:
        - configMapRef:
            name: api-gateway-config
        - secretRef:
            name: api-gateway-secrets
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: api-gateway
spec:
  selector:
    app: api-gateway
  ports:
  - port: 80
    targetPort: 8080
  - port: 443
    targetPort: 8080
  - port: 50051
    targetPort: 50051
  type: ClusterIP
---
apiVersion: v1
kind: Service
metadata:
  name: api-gateway-external
spec:
  selector:
    app: api-gateway
  ports:
  - port: 80
    targetPort: 8080
  - port: 443
    targetPort: 8080
  type: LoadBalancer
```

### Git Module
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: git-module
spec:
  replicas: 2
  selector:
    matchLabels:
      app: git-module
  template:
    metadata:
      labels:
        app: git-module
    spec:
      containers:
      - name: git-module
        image: tardigradeci/git-module:1.0.0
        ports:
        - containerPort: 8080
        volumeMounts:
        - name: git-storage
          mountPath: /data/git
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
  volumeClaimTemplates:
  - metadata:
      name: git-storage
    spec:
      accessModes: [ReadWriteOnce]
      resources:
        requests:
          storage: 100Gi
---
apiVersion: v1
kind: Service
metadata:
  name: git-module
spec:
  selector:
    app: git-module
  ports:
  - port: 80
    targetPort: 8080
```

### Ingress
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: tardigrade-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - tardigrade.example.com
    secretName: tardigrade-tls
  rules:
  - host: tardigrade.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend
            port:
              number: 3000
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: api-gateway
            port:
              number: 8080
```

### Deploy Steps
```bash
# Apply all configurations
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/storage/
kubectl apply -f k8s/secrets/
kubectl apply -f k8s/configmaps/
kubectl apply -f k8s/postgres/
kubectl apply -f k8s/redis/
kubectl apply -f k8s/nats/
kubectl apply -f k8s/minio/
kubectl apply -f k8s/api-gateway/
kubectl apply -f k8s/git-module/
kubectl apply -f k8s/ci-module/
kubectl apply -f k8s/registry-module/
kubectl apply -f k8s/frontend/
kubectl apply -f k8s/ingress/

# Verify
kubectl get pods -n tardigrade-ci
kubectl get svc -n tardigrade-ci
```

---

## 7️⃣ Configuration

### Main Configuration
```yaml
# tardigrade-config.yaml
server:
  http:
    host: 0.0.0.0
    port: 8080
  grpc:
    port: 50051

database:
  git: postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres-git:5432/tardigrade_git
  ci: postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres-ci:5432/tardigrade_ci
  registry: postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres-registry:5432/tardigrade_registry

cache:
  git: redis://redis-git:6379
  ci: redis://redis-ci:6379
  registry: redis://redis-registry:6379

messaging:
  nats: nats://nats:4222

storage:
  minio:
    endpoint: minio:9000
    access_key: ${MINIO_ROOT_USER}
    secret_key: ${MINIO_ROOT_PASSWORD}

security:
  jwt:
    secret: ${JWT_SECRET}
    expiration: 24h
  session:
    secret: ${SESSION_SECRET}
    max_age: 86400
```

---

## 1️⃣2️⃣ Monitoring & Logging

### Prometheus
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
  namespace: tardigrade-ci
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
    scrape_configs:
    - job_name: 'tardigrade-ci'
      static_configs:
      - targets:
        - api-gateway:9090
        - git-module:9090
        - ci-module:9090
```

### Grafana Dashboards
- Tardigrade-CI Overview
- API Gateway Metrics
- Database Performance
- Resource Utilization

---

## 1️⃣3️⃣ Backup & Recovery

### PostgreSQL Backup
```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: tardigrade-ci
spec:
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:15-alpine
            command:
            - /bin/sh
            - -c
            - pg_dump -h postgres-git -U tardigrade tardigrade_git > /backups/git-$(date +%Y%m%d).sql
            volumeMounts:
            - name: backups
              mountPath: /backups
          volumes:
          - name: backups
            persistentVolumeClaim:
              claimName: backup-pvc
```

---

## 📚 Resources
- [Documentation](https://docs.tardigrade-ci.dev)
- [Issues](https://github.com/tardigrade-ci/tardigrade/issues)
- [Discord](https://discord.gg/tardigrade-ci)

---
*Last updated: 2026-06-19*
