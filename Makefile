WOT_API_ID="ghcr.io/chipp/wot_api"

install:
	docker-compose down || true
	docker image rm -f $(WOT_API_ID)
	docker pull $(WOT_API_ID)
	docker-compose up -d

action_deploy:
	make install
