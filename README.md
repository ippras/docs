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
