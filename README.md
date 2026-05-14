# Translate

## Generating the PO Template

```sh
MDBOOK_OUTPUT='{"xgettext": {}}' \
  mdbook build -d po
```

```sh
MDBOOK_OUTPUT='{"xgettext": {"depth": 3}}' \
  mdbook build -d po
```

```sh
for language in en ru; do
    echo "::group::Building $language translation"
    # Set language and adjust site URL. Clear the redirects
    # since they are in sync with the source files, not the
    # translation.
    MDBOOK_BOOK__LANGUAGE=$language \
    mdbook build -d book/$language
    echo "::endgroup::"
done
```

## Initialize a New Translation

```sh
msginit -i po/messages.pot -l ru -o po/ru.po --no-translator
```

```sh
msginit -i po/messages.pot -l ru -o po/ru.po
```

kgv@users.noreply.github.com

## Updating an Existing Translation

msgmerge --update po/ru.po po/messages.pot

msgfmt --statistics po/ru.po

```
MDBOOK_BOOK__LANGUAGE=ru mdbook build
```

```
MDBOOK_BOOK__LANGUAGE=ru mdbook serve
```

```
msgcat po/ru.po -o po/ru.po
```

```
msgcat --no-wrap po/ru.po -o po/ru.po
```

- [Chen et al., 2020](https://doi.org/10.3390/ijms21165695 "Nutritional Indices for Assessing Fatty Acids: A Mini-Review")
- [Dal Bosco et al., 2022](https://doi.org/10.3390/nu14153110 "Indexing of Fatty Acids in Poultry Meat for Its Characterization in Healthy Human Nutrition: A Comprehensive Application of the Scientific Literature and New Proposals")
- [Kralik et al., 2025](https://doi.org/10.18047/poljo.31.1.8 "A Fatty acid profile and THE Health Lipid Indices in Table Eggs")
