#!/usr/bin/env bash
set -euo pipefail

# Documentation of default environment variables in GitHub actions:
# https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/store-information-in-variables#default-environment-variables
#
if test "$GITHUB_EVENT_NAME" = "pull_request"
then
    # We're running in CI against a "pull_request" event, where GitHub creates
    # a weird merge commit that git is pointing to. Determine which commit we
    # actually need to check first.
    git fetch # otherwise name of branch won't be recognized
    commit_to_check="origin/$GITHUB_HEAD_REF"
else
    # We're either not running in CI at all or on a "push" event.
    # The git HEAD is already pointing at the right thing.
    commit_to_check="$(git rev-parse HEAD)"
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
