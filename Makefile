all: bootstap
	./gradlew build --info

bootstap:
	bash .ci/bootstrap_opensuse.sh


docker_run:
	bash ./docker_run.sh
