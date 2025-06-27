#!/bin/sh

set -eux

cp surfboard-scraper.service /lib/systemd/system/
systemctl stop surfboard-scraper
systemctl daemon-reload
systemctl start surfboard-scraper
systemctl enable surfboard-scraper
systemctl status surfboard-scraper
