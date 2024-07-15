# Item Interface Descriptions

## Basic Types

```ts
type integer = number;
```

An `integer` is a number which is a (positive or negative) integer.

```ts
type Uuid = string;
```

A `Uuid` is a string that matches the format specified by [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html).

```ts
type DateTime = string;
```

A `DateTime` is a string that matches the format specified by [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) for a combined date and time, which is indicated in UTC.

```ts
type Duration = string;
```

A `Duration` is a string that matches the format specified by [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) for a duration/period.

```ts
type UrlMatcher = string;
```
A `UrlMatcher` is a string that matches a URL matcher (TODO: extend URL syntax appropriately)

```ts
type ContentType = string;
```

A `ContentType` is a string that matches a Media Type as specified by [RFC 2046](https://www.rfc-editor.org/rfc/rfc2046.html).

```ts
type base64 = string;
```

A `base64` is a string that contains base64 data using the standard alphabet with optional tail padding.

## Generic Items

All items stored by passman implement the `Item` interface.

```ts
interface Item{
    type: ContentType,
    display_name?: string,
}
```

`type` is the `Content-Type` of the `Item` without the format suffix.

`display_name` is an optional property that contains freeform text. The text is intended to be rendered on the client side verbatim.

## Vault

A vault stores a collection of items

```ts
interface Vault extends Item{
    type: "application/x-passman-vault",
    content: Array<Uuid>
}
```

`content` is an array of `uuid`s that refer to the vault.


## Website Items

Certain items apply to a specific website/url.

```ts
interface UrlItem extends Item{
    url: url_matcher,
}
```

## Password Item

A Password Item stores a password for a website, and typically a User ID (such as username or email address) for authentication.

```ts
interface PasswordItem extends UrlItem{
    type: "application/x-passman-login-password"
    login_id?: string,
    login_password: string,
}
```

`login_id` is the optional ID used for logging into the appropriate website.

`login_password` is the password used for logging into that website.

## TOTP Item

```ts
enum Algorithm{
    SHA1 = "sha1",
    SHA256 = "sha256",
}
```

A `TOTP` item contains the information needed to generate Time-based One Time Passwords, according to [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238) 

```ts
interface TotpGeneratorItem extends WebsiteItem{
    type: "application/x-passman-totp-generator",
    alg: Algorithm,
    totp_epoch?: DateTime,
    totp_step: Duration,
    totp_digits: integer,
    totp_key: base64
}
```

`alg` is the hash algorithm used to generate the ToTP token.

`totp_epoch` is the epoch of the TOTP Token. Optional and defaults to the Unix Epoch "1970-01-01T00:00:00Z".

`totp_step` is the duration between TOTP Token steps (new tokens).

`top_digits` is the number of digits used for the resulting TOTP Token.

`totp_key` is the base64 encoded key for TOTP.

