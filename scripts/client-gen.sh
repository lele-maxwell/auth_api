#!/bin/bash

set -e

# Ensure output dir exists
mkdir -p ts-client

# Run the OpenAPI Generator (you can use docker too)
openapi-generator-cli generate \
  -i openapi.json \
  -g typescript-fetch \
  -o ts-client \
  --skip-validate-spec

# Optional: run prettier or ts cleanup if needed
cd ts-client
npm install
npm run build
