CARGO_PROFILE?=dev

.PHONY: build test clean package

build:
	cross build --target armv7-unknown-linux-musleabihf --profile ${CARGO_PROFILE}

test:
	cargo test

clean:
	cross clean
	rm surfboard_scraper.tar.gz

TMP_DIR := $(shell mktemp -d)
package: build
	echo "Packaging"
	mkdir -p ${TMP_DIR}/surfboard_scraper/deploy
	cp deploy/surfboard-scraper.service ${TMP_DIR}/surfboard_scraper/
	cp deploy/install.sh ${TMP_DIR}/surfboard_scraper/
	cp deploy/config.json ${TMP_DIR}/surfboard_scraper/deploy/
	cp target/armv7-unknown-linux-musleabihf/release/surfboard_scraper ${TMP_DIR}/surfboard_scraper/
	tar --no-xattrs -czvf surfboard_scraper.tar.gz -C ${TMP_DIR} .
	rm -rf ${TMP_DIR}

deploy-scraper: package
	scp surfboard_scraper.tar.gz deploy/install.sh pi@192.168.4.79:~
	ssh pi@192.168.4.79 -t 'sudo bash install.sh'
