@echo off

type NUL && "%CODEQL_DIST%\codeql.exe" database index-files ^
    --prune=**/*.testproj ^
    --include-extension=.inc ^
    --include-extension=.php ^
    --include-extension=.php3 ^
    --include-extension=.php4 ^
    --include-extension=.php5 ^
    --include-extension=.phtml ^
    --size-limit=5m ^
    --language=php ^
    --working-dir=. ^
    "%CODEQL_EXTRACTOR_PHP_WIP_DATABASE%"

exit /b %ERRORLEVEL%
