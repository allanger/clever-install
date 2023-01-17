#!/bin/bash
echo 'renaming clin to clin-$VERSION-$SYSTEM format'
mkdir -p release
echo "version - $clin_VERSION"
for BUILD in build*; do
  SYSTEM=$(echo $BUILD | sed -e 's/build-//g')
  echo "system - $SYSTEM"
  cp $BUILD/clin release/clin-$CLIN_VERSION-$SYSTEM
done
ls release
