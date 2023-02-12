#!/bin/bash
echo 'renaming dudo to dudo-$VERSION-$SYSTEM format'
mkdir -p release
echo "version - $CLIN_VERSION"
for BUILD in build*; do
  SYSTEM=$(echo $BUILD | sed -e 's/build-//g')
  echo "system - $SYSTEM"
  cp $BUILD/dudo release/dudo-$CLIN_VERSION-$SYSTEM
done
ls release
