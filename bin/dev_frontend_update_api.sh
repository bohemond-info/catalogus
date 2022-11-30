#!/usr/bin/env zsh
set -o nounset
set -o errexit
set -o pipefail

BE=$1
FE=$2

# Check for toolchain dependencies
if ! command -v typegen &> /dev/null
then
    printf "'typegen' is required but not installed. Try: \n\t yarn global add openapi-client-axios-typegen\n"
    exit 1
fi

# Runs the typegen to pull the Openapi spec from the backend app and make it available to the frontend

SCRIPT_DIR="${0:a:h}"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WEB_API_DIR="$(cd "$ROOT_DIR/$BE" && pwd)"
WEB_FEA_DIR="$(cd "$ROOT_DIR/$FE" && pwd)"

OPENAPI_SPEC="openapi.json"
OPENAPI_FILE="${WEB_API_DIR}/resources/${OPENAPI_SPEC}"
TYPES_FILE="${WEB_FEA_DIR}/src/types/backend_api.d.ts"

printf "Generating OpenAPI Types for %s Frontend. Assumes current specification at:\n\t%s\n" "${FE}" "${OPENAPI_FILE}"
GENERATED_WARNING="// GENERATED CODE\n// Do not modify directly. Run from repo root:\n//    bin/dev_frontend_update_api.sh <backend> <frontend>\n// Or:\n//    yarn update-api"
echo -e "${GENERATED_WARNING}\n$(typegen "$OPENAPI_FILE")" > "$TYPES_FILE"
printf "Generated types file at:\n\t%s\n" "${TYPES_FILE}"
