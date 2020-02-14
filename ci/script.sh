# This script takes care of testing your crate

set -ex

# DONE This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --features serde
    cross build --target $TARGET --release
    cross build --target $TARGET --release --features serde

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross clean
    cross test --target $TARGET
    cross test --target $TARGET --features serde
    cross clean
    cross test --target $TARGET --release
    cross test --target $TARGET --release --features serde

    # cross run --target $TARGET
    # cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
