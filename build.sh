for language in $LANGUAGES; do
    echo "::group::Building $language translation"
    # Set language and adjust site URL. Clear the redirects
    # since they are in sync with the source files, not the
    # translation.
    MDBOOK_BOOK__LANGUAGE=$language \
    mdbook build -d book/$language
    echo "::endgroup::"
done