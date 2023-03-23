# cookie-csr

This example shows how to save the preferred language in a CSRed Leptos app using a browser cookie.

## Details

### Disadvantages

- At the initial load the language is not known. So the first render will be in the default language, and then the user need to change the language to their preferred one.
- Browser cookies can be disabled.
- Browser cookies are not consistent, because all don't treat non ASCII characters the same way. See https://stackoverflow.com/a/1969339/9167585. To prevent this problem in this particular example we are using a very dirty workaround urlencoding the `ñ` character for the Spanish language in the cookie.

### Advantages

- Very simple, we don't need to deal with ISO language codes, the readable language name is the identifier for itself.
