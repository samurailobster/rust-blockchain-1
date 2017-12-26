#!/bin/sh

FILES=$(find target/release -maxdepth 1 -name "blockchain_*" -perm +111 -type f)

for file in $FILES
do
  echo "Current file: $file"
  echo "Size: $(du -h $file | cut -f -1)"
  echo "Stripping, just for you"
  strip $file
  echo "Done. New size: $(du -h $file | cut -f -1)"
  echo ""
done