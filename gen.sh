docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://discoveryprovider.audius.co/v1/swagger.json \
    -g rust \
    -o /local/src/api \
    --skip-validate-spec \
    --global-property models,supportingFiles