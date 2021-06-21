
# Do this
migrate: stop move-db backup user ssh
	cp ~/.0L/Makefile /home/val/
	@runuser -l  val -c 'mkdir -p ~/.0L/'
	@runuser -l  val -c 'make path bin install template reset'
	# TODO need to install cargo and deps


## do this from previous user.
stop:
	ol mgmt --stop all

user:
	useradd -m val | true

ssh:
	mkdir -p /home/val/.ssh
	cp /root/.ssh/authorized_keys /home/val/.ssh/
	chown -R val /home/val/.ssh/

move-db:
	mv ~/.0L/db /home/val/.0L/db


## Do below as user: val
path:
	@if (cat ~/.bashrc | grep '~/bin:') ; then \
		echo PATH already contains ~/bin ; \
	else \
		echo adding to PATH ; \
		echo PATH=~/bin:$$PATH >> ~/.bashrc ; \
	fi

mv-bin:
	@if which ol | grep /usr/local/bin/ ; then \
		echo copy all bins ; \
		mkdir ~/bin/ | true ; \
		mv /usr/local/bin/* ~/bin/ ; \
	fi

ap-template:
	curl https://raw.githubusercontent.com/LOL-LLC/donations-record/main/clean.autopay_batch.json --output ~/.0L/template.autopay_batch.json


reset:
	onboard val --skip-mining --upstream-peer http://167.172.248.37/ --source-path ~/libra


backup:
	cd ~ && rsync -av --exclude db/ --exclude logs/ ~/.0L ~/0L_backup_$(shell date +"%m-%d-%y")