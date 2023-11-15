cd extractor
cargo build --release
cd ..

extractor\target\release\codeql-extractor-php -- generate --dbscheme ql/lib/php.dbscheme --library ql/lib/codeql/php/ast/internal/TreeSitter.qll

codeql query format -i ql\lib\codeql/php\ast\internal\TreeSitter.qll

rm -Recurse -Force extractor-pack
mkdir extractor-pack | Out-Null
cp codeql-extractor.yml, ql\lib\php.dbscheme extractor-pack
cp -Recurse tools extractor-pack
cp -Recurse downgrades extractor-pack
mkdir extractor-pack\tools\win64 | Out-Null
cp extractor\target\release\codeql-extractor-php.exe extractor-pack\tools\win64\extractor.exe
