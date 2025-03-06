rm -rf ./assets_for_test/assets && echo "removed old assets"
mkdir ./assets_for_test/assets
cp ./assets_for_test/source/* ./assets_for_test/assets/ && echo "created new assets"
echo "------------------------------"