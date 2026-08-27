CARGO_PROFILE?=release
BUILD_PROFILE=$(if $(filter dev,$(CARGO_PROFILE)),debug,$(CARGO_PROFILE))

# 64-bit Raspberry Pi target, built via Apple `container` (no Docker required).
TARGET=aarch64-unknown-linux-musl
BUILDER_IMAGE=surfboard-builder

.PHONY: build builder test clean package

# Build the cross-compilation image once; `container build` caches layers.
builder:
	container build -t ${BUILDER_IMAGE} -f deploy/Containerfile .

build: builder
	container run --rm -m 8g -c 8 -v $(CURDIR):/project -w /project ${BUILDER_IMAGE} \
		cargo build --target ${TARGET} --profile ${CARGO_PROFILE}

test:
	cargo test

clean:
	cargo clean
	rm -f surfboard_scraper.tar.gz

TMP_DIR := $(shell mktemp -d)
package: build
	echo "Packaging"
	mkdir -p ${TMP_DIR}/surfboard_scraper/deploy
	cp deploy/surfboard-scraper.service ${TMP_DIR}/surfboard_scraper/
	cp deploy/install.sh ${TMP_DIR}/surfboard_scraper/
	mkdir -p ${TMP_DIR}/surfboard_scraper/deploy/configs
	cp deploy/configs/*.json ${TMP_DIR}/surfboard_scraper/deploy/configs/
	cp deploy/photo.png ${TMP_DIR}/surfboard_scraper/deploy/
	cp target/${TARGET}/${BUILD_PROFILE}/surfboard_scraper ${TMP_DIR}/surfboard_scraper/
	COPYFILE_DISABLE=1 tar --no-xattrs --exclude='._*' -czvf surfboard_scraper.tar.gz -C ${TMP_DIR} .
	rm -rf ${TMP_DIR}

deploy-scraper: package
	scp surfboard_scraper.tar.gz deploy/install.sh pi@192.168.4.79:~
	ssh pi@192.168.4.79 -t 'sudo bash install.sh'
