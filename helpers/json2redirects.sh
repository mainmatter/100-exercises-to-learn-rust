#!/bin/bash

# Ensure the JSON file is provided as an argument
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <input_json_file>"
  exit 1
fi

input_file=$1

# Use jq to parse the JSON and format the output
jq -r 'to_entries[] | "/" + .value + " " + .key' "$input_file"
