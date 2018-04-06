# rs-view-tracker

A really simple view tracker that allows you to track your users
while respecting their privacy.

## Data collected

The only data collected at the moment is page views, namely the time
and page viewed.

All collected data is saved in CSV, with one file for every day. This
allows easy manual inspection and simple processing.

## How to use it

Compile the tracker with `cargo build --release`. Create a configuration
file at `/etc/rs-view-tracker.conf`. TOML syntax is used, and the only
required parameter is `logs_dir`. The example file may look like this:

```toml
port = 3000
logs_dir = "/var/log/rs-view-tracker"
timezone = "Europe/Warsaw"
```

Then, add a HTML element that forces browser to fetch a resource
to all pages of your site that you want to track. For example,

```html
<script src="tracker.yoursite.com:3000/script.js"></script>
```

The tracker response will always be empty, so no JavaScript will
be actually executed.
