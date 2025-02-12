#!/usr/bin/env bash
set -euo pipefail

echo "Checking commit message for 🥳 emoji..."
if ! git log -1 --pretty=%B | grep -qE "(:partying_face:|🥳)"
then
    echo "Please be more joyful !! 🥳"
    exit 1
fi

echo "Checking commit message subject length..."
if (( "$(git log -1 --pretty=%s | wc -c)" > 50 ))
then
    echo "Please avoid lengthy commit message subjects!"
    echo "https://cbea.ms/git-commit/#limit-50"
    exit 1
fi
