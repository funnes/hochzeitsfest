#!/usr/bin/env bash
set -euo pipefail

# commit to check: first arg or HEAD
commit_to_check="${1:-$(git rev-parse HEAD)}"

if test -n "${CI+x}" ; then
    # GitHub CI may not have this ref fetched by default
    git fetch origin "$commit_to_check"
fi

echo "Checking commit message for 🥳 emoji..."
if ! git log -1 --pretty=%B "$commit_to_check" | grep -qE "(:partying_face:|🥳)"
then
    echo "Please be more joyful !! 🥳"
    exit 1
fi

echo "Checking commit message subject length..."
if (( "$(git log -1 --pretty=%s "$commit_to_check" | wc -c)" > 50 ))
then
    echo "Please avoid lengthy commit message subjects!"
    echo "https://cbea.ms/git-commit/#limit-50"
    exit 1
fi
