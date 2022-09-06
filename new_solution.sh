#!/bin/bash

# shellcheck disable=SC2001,SC2129

RAW_PROBLEM_NAME=$1
PROBLEM_NAME=$(echo "$RAW_PROBLEM_NAME" | sed -e 's/-/_/g')
if [[ $PROBLEM_NAME == "" ]]; then
  echo "PLEASE INPUT THE PROBLEM NAME"
  exit 1
fi

FILENAME=
PROBLEM_ID=$2
if [[ $PROBLEM_ID != "" ]]; then
  FILENAME="${PROBLEM_NAME}_${PROBLEM_ID}"
else
  FILENAME=$PROBLEM_NAME
fi

if [[ -f "src/solutions/$FILENAME.rs" ]]; then
  echo "ERROR: ${FILENAME} already exist"
  exit 1
fi

touch "src/solutions/$FILENAME.rs"
printf "pub mod %s;\n" "$FILENAME" >> src/solutions/mod.rs
printf "// PROBLEM: https://leetcode.cn/problems/%s\n" "$RAW_PROBLEM_NAME" >> "src/solutions/$FILENAME.rs"
printf "// DATE:    %s\n\n\n\n" "$(date "+%Y/%m/%d %H:%M:%S %z")" >> "src/solutions/$FILENAME.rs"
printf "#[cfg(test)]\n" >> "src/solutions/$FILENAME.rs"
printf "use crate::utils::test::TestCase;\n\n" >> "src/solutions/$FILENAME.rs"
printf "fn test_solution() {}" >> "src/solutions/$FILENAME.rs"