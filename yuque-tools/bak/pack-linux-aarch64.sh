if [[ $(echo $0 | awk '/^\//') == $0 ]]; then
    ABSPATH=$(dirname $0)
else
    ABSPATH=$PWD/$(dirname $0)
fi
cd ${ABSPATH}
cross build --release --target aarch64-unknown-linux-gnu	