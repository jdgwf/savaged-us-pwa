deploy:
	trunk build --release
	@ssh jeff@savaged.us 'cd /srv/www/v4-rust.savaged.us && rm -Rf *'
	cp robots.txt public
	@rsync -amrv --exclude='/.env' --exclude='/package-lock.json' --exclude='/.git/' --exclude='/tmp/' --exclude='/data/' --exclude='/node_modules/' --stats -e ssh public/* jeff@savaged.us:/srv/www/v4-rust.savaged.us/public

