
.PHONY: deploy

deploy:
	cargo web deploy
	butler push ./target/deploy fasterthanlime/lifeclick:html5
