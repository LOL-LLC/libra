SHELL=/usr/bin/env bash
DATA_PATH = ${HOME}/temp
SWARM_TEMP = ${DATA_PATH}/swarm_temp
LOG=${DATA_PATH}/test-autopay.log
UNAME := $(shell uname)

NODE_ENV=test
TEST=y

ifndef SOURCE_PATH
SOURCE_PATH = ${HOME}/libra
endif
MAKE_FILE = ${SOURCE_PATH}/ol/integration-tests/test-fullnode.mk

# alice
ifndef PERSONA
PERSONA=alice
endif

BOB_LOG = ${SWARM_TEMP}/logs/1.log
MNEM="talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse"

NUM_NODES = 2

START_TEXT = "To run the Libra CLI client"
SUCCESS_TEXT = "transaction executed"
REMOVE_SUCCESS_TEXT = "The validator is not in the validator set"

# test: clean swarm check-swarm stop
test: clean swarm check-swarm leave-set check-leave

clean:
	rm -rf ${DATA_PATH} | true
	mkdir -p ${SWARM_TEMP}

swarm:
	@echo Building Swarm
	cd ${SOURCE_PATH} && cargo build -p libra-node -p cli
	cd ${SOURCE_PATH} && cargo run -p libra-swarm -- --libra-node ${SOURCE_PATH}/target/debug/libra-node -c ${SWARM_TEMP} -n ${NUM_NODES} &> ${LOG} &

stop:
	killall libra-swarm libra-node miner ol | true

init:
	cd ${SOURCE_PATH} && cargo r -p ol -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} init --source-path ${SOURCE_PATH}

tx:
	cd ${SOURCE_PATH} && NODE_ENV=test TEST=y cargo r -p txs -- --swarm-path ${SWARM_TEMP} --swarm-persona ${PERSONA} val-set --leave



check-swarm: 
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${START_TEXT} ${LOG} ; then \
				break; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done

leave-set: 
	PERSONA=bob make -f ${MAKE_FILE} init
	PERSONA=bob make -f ${MAKE_FILE} tx

check-leave: 
	@while [[ ${NOW} -le ${END} ]] ; do \
			if grep -q ${REMOVE_SUCCESS_TEXT} ${BOB_LOG} ; then \
				break; \
			else \
				echo . ; \
			fi ; \
			echo "Sleeping for 5 secs" ; \
			sleep 5 ; \
	done