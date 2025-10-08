help:
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

init: ## Init
	docker network create -d bridge key-value-compare

up: infra-up apps-up ## Up
down: infra-down apps-down ## Down
stop: infra-stop apps-stop ## Stop
ps: ## View all docker containers
	docker ps | grep pbs-tools

apps-up: ## Applications. Up
apps-down: ## Applications. Down
apps-stop: ## Applications. Stop

infra-up: picodata-up ## Infra. Up
infra-down: picodata-down ## Infra. Down
infra-stop: picodata-stop ## Infra. Stop

picodata-build: ## Picodata. Build
	cd .docker/picodata && docker compose build
picodata-up: ## Picodata. Up
	cd .docker/picodata && docker compose up
picodata-upd: ## Picodata. Up as deamon
	cd .docker/picodata && docker compose up -d
picodata-down: ## Picodata. Down
	cd .docker/picodata && docker compose down -v
picodata-stop: ## Picodata. Stop
	cd .docker/picodata && docker compose stop
picodata-logs: ## Picodata. Logs
	cd .docker/picodata && docker compose logs -tf picodata
picodata-bash: ## Picodata. Bash
	cd .docker/picodata && docker compose exec -it picodata bash
