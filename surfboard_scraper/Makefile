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
	mkdir ${TMP_DIR}/surfboard_scraper
	cp deploy/surfboard-scraper.service ${TMP_DIR}/surfboard_scraper/
	cp deploy/install.sh ${TMP_DIR}/surfboard_scraper/
	cp target/armv7-unknown-linux-musleabihf/release/surfboard_scraper ${TMP_DIR}/surfboard_scraper/
	tar --no-xattrs -czvf surfboard_scraper.tar.gz -C ${TMP_DIR} .
	rm -rf ${TMP_DIR}

deploy-scraper: package
	scp surfboard_scraper.tar.gz pi@192.168.4.79:~
	ssh pi@192.168.4.79 -t 'tar zxvf surfboard_scraper.tar.gz && cd surfboard_scraper && sudo ./install.sh'
