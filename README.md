# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

What are we building?
The features of HotDog are fairly simple:

[-] Engage with a stream of cute dog photos
[-] Swipe right if we want to save the dog photo to our collection
[-] Swipe left if we don't want to save the dog photo
[-] View the dog photos we saved later
[-] At the end of the tutorial, you'll have your very own HotDog app to remix and download to your device.
