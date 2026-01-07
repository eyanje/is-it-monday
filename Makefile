all: backend frontend

REPOSITORY=harbor.eyanje.net/is-it-monday

.PHONY: backend
backend:
	docker build --file backend/Dockerfile --tag ${REPOSITORY}/backend

.PHONY: frontend
frontend:
	docker build --file frontend/Dockerfile --tag ${REPOSITORY}/frontend

.PHONY: backend.push
.PHONY: frontend.push

backend.push: backend
	docker push ${REPOSITORY}/backend:latest

frontend.push: frontend
	docker push ${REPOSITORY}/frontend:latest

push: backend.push frontend.push
