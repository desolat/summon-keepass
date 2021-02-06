#!/usr/bin/env bash

set -e
set -o pipefail

# Install the latest release

error() {
  echo "ERROR: $@" 1>&2
  echo "Exiting installer" 1>&2
  exit 1
}

PROJECT="summon-keepass"
API_BASE_URL="https://api.github.com/repos/desolat/summon-keepass"
PROJECT_BASE_URL="https://github.com/desolat/summon-keepass"

ARCH=`uname -m`
if [ "${ARCH}" != "x86_64" ]; then
  error "$PROJECT only available for 64-bit systems"
fi

KERNEL_NAME=`uname | tr "[:upper:]" "[:lower:]"`

if [ "${KERNEL_NAME}" != "linux" ]; then # && [ "${KERNEL_NAME}" != "darwin" ]
  error "This installer currently only supports Linux"
fi

tmp="/tmp"
if [ ! -z "$TMPDIR" ]; then
  tmp=$TMPDIR
fi

# secure-ish temp dir creation without having mktemp available (DDoS-able but not exploitable)
tmp_dir="$tmp/install.sh.$$"
(umask 077 && mkdir $tmp_dir) || exit 1

# do_download URL DIR
do_download() {
  echo "Downloading $1"
  if [[ $(command -v wget) ]]; then
    wget -q -O "$2" "$1" >/dev/null
  elif [[ $(command -v curl) ]]; then
    curl --fail -sSL -o "$2" "$1" &>/dev/null || true
  else
    error "Could not find wget or curl"
  fi
}

# Get latest release from GitHub API
get_latest_version() {
  local LATEST_VERSION_URL="$API_BASE_URL/releases/latest"
  local latest_payload

  if [[ $(command -v wget) ]]; then
    latest_payload=$(wget -q -O - "$LATEST_VERSION_URL")
  elif [[ $(command -v curl) ]]; then
    latest_payload=$(curl --fail -sSL "$LATEST_VERSION_URL")
  else
    error "Could not find wget or curl"
  fi

  echo "$latest_payload" |
    grep '"tag_name":' | # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/' # Pluck JSON value
}

LATEST_VERSION=$(get_latest_version)
echo "Latest version: $LATEST_VERSION"

FILE_NAME="$PROJECT-${KERNEL_NAME}-amd64.tar.gz"
URL="${PROJECT_BASE_URL}/releases/download/${LATEST_VERSION}/$FILE_NAME"

FILE_PATH="${tmp_dir}/$FILE_NAME"
do_download ${URL} ${FILE_PATH}

TARGET_PATH="/usr/local/bin"
echo "Installing $PROJECT ${LATEST_VERSION} into $TARGET_PATH"

if [[ "$FILE_PATH" == *.tar.gz]]; then
  if sudo -h >/dev/null 2>&1; then
    sudo tar -C $TARGET_PATH -o -zxvf ${FILE_PATH} >/dev/null
  else
    tar -C $TARGET_PATH -o -zxvf ${FILE_PATH} >/dev/null
  fi
else 
  cp $FILE_PATH "${TARGET_PATH}"
fi

echo "Installed $PROJECT to $TARGET_PATH"
