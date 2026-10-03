#!/usr/bin/env bash
# generate_next_id.sh
# Reads INDEX.md, finds the highest PLAN-NNNN, and outputs the next ID.

INDEX_FILE="../../docs/plans/INDEX.md"

# Default to 0 if no plans exist yet
MAX_ID=0

if [ -f "$INDEX_FILE" ]; then
    # Grep for PLAN-NNNN, extract the number, sort numerically and get the highest
    HIGHEST_PLAN=$(grep -o "PLAN-[0-9]\{3,4\}" "$INDEX_FILE" | grep -o "[0-9]\+" | sort -n | tail -1)
    
    if [ ! -z "$HIGHEST_PLAN" ]; then
        # Remove leading zeros by forcing base 10 arithmetic
        MAX_ID=$((10#$HIGHEST_PLAN))
    fi
fi

NEXT_ID=$((MAX_ID + 1))
printf "PLAN-%04d\n" $NEXT_ID
