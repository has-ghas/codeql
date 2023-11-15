#!/bin/sh

set -eu

exec "${CODEQL_DIST}/codeql" database index-files \
    --prune="**/*.testproj" \
    --include-extension=.php \
    --include-extension=.php3 \
    --include-extension=.php4 \
    --include-extension=.php5 \
    --include-extension=.phtml \
    --include-extension=.inc \
    --include="**/Gemfile" \
    --size-limit=5m \
    --language=php \
    --working-dir=.\
    "$CODEQL_EXTRACTOR_PHP_WIP_DATABASE"
