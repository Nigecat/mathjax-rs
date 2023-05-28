rm -rv temp
rm -v data.zip
mkdir -vp temp
mkdir -vp temp/node_modules
cd ../mathjax
npm ci
npm run compile
cp -rva js/. ../mathjax-data/temp
cp -rva node_modules/mhchemparser ../mathjax-data/temp/node_modules
cd ../mathjax-data
(cd temp && zip -r ../data.zip *)
rm -rv temp
