# navigator-languages-localstorage-csr

This example shows how to save the preferred language in a CSRed Leptos app using the browser localstorage. The initial language, if not previously defined in the cookie, is determined by the browser's `navigator.languages` property.

## Details

See all the differences between local storage vs cookies: https://stackoverflow.com/a/67508483/9167585

### Disadvantages

- Local storage is disabled when cookies are disabled in most browsers, see https://stackoverflow.com/a/31164410/9167585
- Differing from the cookies-only example, you need to activate the `Storage` feature of web-sys.
- There are no easy preventions for Local Storage from XSS attack.

### Advantages

- The initial language is determined by the browser's `navigator.languages` property.
- The manipulation of localstorage seem to be more straight forward than cookies.
- Not vulnerable to CSRF (Cross Site Request Forgery).
