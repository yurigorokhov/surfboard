#!/bin/sh

set -eux

systemctl disable surfboard-scraper
systemctl stop surfboard-scraper

tar zxvf surfboard_scraper.tar.gz -C /opt
cp /opt/surfboard_scraper/surfboard-scraper.service /etc/systemd/system/

systemctl daemon-reload
systemctl start surfboard-scraper
systemctl enable surfboard-scraper
systemctl status surfboard-scraper
