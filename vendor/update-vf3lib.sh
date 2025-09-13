#!/bin/bash
# Script to update vendored vf3lib headers

set -e

echo "Updating vendored vf3lib headers..."

# Create secure temporary directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Clone latest vf3lib
git clone --depth 1 https://github.com/MiviaLab/vf3lib.git "$TEMP_DIR/vf3lib"

# Get commit info
pushd "$TEMP_DIR/vf3lib" > /dev/null
COMMIT=$(git rev-parse HEAD)
DATE=$(git log -1 --format="%cI")
MESSAGE=$(git log -1 --format="%s")
popd >/dev/null

# Update headers
rm -rf vendor/vf3lib/include
cp -r "$TEMP_DIR/vf3lib/include" vendor/vf3lib/

# Update VERSION.txt
cat > vendor/vf3lib/VERSION.txt << EOF
UpstreamRepo: https://github.com/MiviaLab/vf3lib
Commit: $COMMIT
Date: $DATE
Message: $MESSAGE
EOF

echo "Updated to vf3lib commit: $COMMIT"
echo "Date: $DATE"
echo "Message: $MESSAGE"
