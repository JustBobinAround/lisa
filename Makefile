build-container:
	make -C container docker

open-container:
	docker run --rm -it -v ${PWD}:/work -w /work ks-rs

run:
	docker run --rm -it -v ${PWD}:/work -w /work ks-rs /bin/bash -c "cargo run"
