build:
	#!/usr/bin/env bash
	set -euxo pipefail

	mkdir -p build
	for file in *.typ ; do
		just _compile $file
	done

_compile file:
	typst compile --font-path fonts {{file}} build/{{without_extension(file)}}.pdf

clean:
	rm -rf build

check-commit-message:
	devel/scripts/check_commit_message.sh
