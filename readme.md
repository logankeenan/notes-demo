# RORA - Notes Demo

Contains the core app logic for a very basic notes app using the [tide](https://github.com/http-rs/tide) server
framework. All data is saved to an [API](https://github.com/rora-rs/notes-demo-api). This app allows users to view all
notes, view an individual note, and edit a note. The note can contain markdown. This app is consumed on the following
platforms:

* Server-side [notes-demo-server](https://github.com/rora-rs/notes-demo-server)
* Cloudflare Worker [notes-demo-cf-worker](https://github.com/rora-rs/notes-demo-cf-worker)
* Browser [notes-demo-spa](https://github.com/rora-rs/notes-demo-spa)
* Electron (_coming soon_)
* iOS (_coming soon_)
* Android (_coming soon_)