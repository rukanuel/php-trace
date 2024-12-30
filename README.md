# PHP Trace
PHP Trace is a native PHP extension designed to efficiently retrieve Cloudflare's trace data in a straightforward manner.

For installation instructions, see [INSTALL.md](INSTALL.md).

## Features
- Provides a simple way to access Cloudflare's trace data from PHP.
- Lightweight and efficient native extension.

## Usage
```php
print_r(trace());
print_r(trace("country"));
```
For full returned array see [return.json](return.json)